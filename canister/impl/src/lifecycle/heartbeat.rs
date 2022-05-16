use crate::model::ledger_sync_state::TryStartSyncResult;
use crate::model::ledger_sync_state::Version;
use crate::model::notifications::Notification;
use crate::{mutate_state, State, Subscriptions};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::heartbeat;
use ic_ledger_types::{
    AccountIdentifier, ArchivedBlocksRange, Block, BlockIndex, GetBlocksArgs, GetBlocksResult,
    Operation,
};
use itertools::Itertools;
use std::collections::HashSet;
use tracing::error;
use transaction_notifier::NotifyTransactionArgs;
use types::CanisterId;

#[heartbeat]
fn heartbeat() {
    sync_ledger_transactions::run();
    push_notifications::run();
}

mod sync_ledger_transactions {
    use super::*;

    struct TokenToSync {
        token_symbol: String,
        ledger_canister_id: CanisterId,
        from_block: BlockIndex,
        version: Version,
    }

    pub fn run() {
        let tokens_to_sync = mutate_state(tokens_to_sync);
        if !tokens_to_sync.is_empty() {
            ic_cdk::spawn(sync_tokens(tokens_to_sync));
        }
    }

    fn tokens_to_sync(state: &mut State) -> Vec<TokenToSync> {
        let now = state.env.now();

        state
            .data
            .tokens
            .values_mut()
            .filter_map(|t| {
                if let TryStartSyncResult::Success(block_index_synced_up_to, version) =
                    t.ledger_sync_state_mut().try_start(now)
                {
                    Some(TokenToSync {
                        token_symbol: t.token_symbol().to_string(),
                        ledger_canister_id: t.ledger_canister_id(),
                        from_block: block_index_synced_up_to + 1,
                        version,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    async fn sync_tokens(tokens_to_sync: Vec<TokenToSync>) {
        futures::future::join_all(tokens_to_sync.into_iter().map(sync_token)).await;
    }

    async fn sync_token(token_to_sync: TokenToSync) {
        let mut new_block_index_synced_up_to = None;
        let mut success = false;

        match blocks_since(
            token_to_sync.ledger_canister_id,
            token_to_sync.from_block,
            1000,
        )
        .await
        {
            Ok(blocks) => {
                if !blocks.is_empty() {
                    mutate_state(|state| {
                        new_block_index_synced_up_to =
                            Some(token_to_sync.from_block + (blocks.len() as u64) - 1);

                        enqueue_notifications(
                            &token_to_sync.token_symbol,
                            token_to_sync.ledger_canister_id,
                            blocks,
                            token_to_sync.from_block,
                            state,
                        );
                    });
                }
                success = true;
            }
            Err(error) => error!(?error, "Failed to get blocks from ledger"),
        }

        mutate_state(|state| {
            mark_sync_complete(
                &token_to_sync.token_symbol,
                new_block_index_synced_up_to,
                success,
                token_to_sync.version,
                state,
            )
        });
    }

    async fn blocks_since(
        ledger_canister_id: CanisterId,
        start: BlockIndex,
        length: usize,
    ) -> CallResult<Vec<Block>> {
        let response =
            ic_ledger_types::query_blocks(ledger_canister_id, GetBlocksArgs { start, length })
                .await?;

        if response.archived_blocks.is_empty() {
            Ok(response.blocks)
        } else {
            async fn get_blocks_from_archive(
                range: ArchivedBlocksRange,
            ) -> CallResult<GetBlocksResult> {
                let args = GetBlocksArgs {
                    start: range.start,
                    length: range.length as usize,
                };
                let (response,) =
                    ic_cdk::call(range.callback.canister_id, &range.callback.method, (args,))
                        .await?;
                Ok(response)
            }

            // Get the transactions from the archive canisters
            let futures: Vec<_> = response
                .archived_blocks
                .into_iter()
                .sorted_by_key(|a| a.start)
                .map(get_blocks_from_archive)
                .collect();

            let archive_responses = futures::future::join_all(futures).await;

            let results = archive_responses
                .into_iter()
                .collect::<CallResult<Vec<_>>>()?;

            Ok(results
                .into_iter()
                .flat_map(|r| r.unwrap().blocks)
                .chain(response.blocks)
                .collect())
        }
    }

    fn mark_sync_complete(
        token_symbol: &str,
        new_block_index_synced_up_to: Option<BlockIndex>,
        success: bool,
        version: Version,
        state: &mut State,
    ) {
        if let Some(token_data) = state.data.tokens.get_mut(token_symbol) {
            let ledger_sync_state = token_data.ledger_sync_state_mut();

            if let Some(block_index) = new_block_index_synced_up_to {
                ledger_sync_state.set_synced_up_to(block_index, Some(version));
            }

            ledger_sync_state.mark_sync_complete(success, state.env.now());
        }
    }

    fn enqueue_notifications(
        token_symbol: &str,
        ledger_canister_id: CanisterId,
        blocks: Vec<Block>,
        from_block_index: BlockIndex,
        state: &mut State,
    ) {
        let subscriptions = &state.data.subscriptions;

        for (block_index, block) in blocks
            .into_iter()
            .enumerate()
            .map(|(index, block)| ((index as u64) + from_block_index, block))
        {
            let account_identifiers = extract_account_identifiers(&block.transaction.operation);
            let canisters_to_notify =
                extract_canisters_to_notify(&account_identifiers, subscriptions);

            for canister_id in canisters_to_notify {
                state.data.notifications.enqueue(Notification {
                    canister_id,
                    args: NotifyTransactionArgs {
                        token_symbol: token_symbol.to_string(),
                        ledger_canister_id,
                        block_index,
                        block: block.clone(),
                    },
                })
            }
        }
    }

    fn extract_account_identifiers(operation: &Operation) -> Vec<AccountIdentifier> {
        match operation {
            Operation::Transfer { from, to, .. } => vec![*from, *to],
            Operation::Mint { to, .. } => vec![*to],
            Operation::Burn { from, .. } => vec![*from],
        }
    }

    fn extract_canisters_to_notify(
        account_identifiers: &[AccountIdentifier],
        subscriptions: &Subscriptions,
    ) -> HashSet<CanisterId> {
        HashSet::from_iter(
            account_identifiers
                .iter()
                .filter_map(|a| subscriptions.get(a))
                .flatten()
                .copied(),
        )
    }
}

mod push_notifications {
    use super::*;
    use std::cmp::min;

    const MAX_NOTIFICATIONS_PER_BATCH: usize = 5;

    pub fn run() {
        if let Some(batch) = mutate_state(next_batch) {
            ic_cdk::spawn(push_batch(batch));
        }
    }

    struct Batch {
        notifications: Vec<Notification>,
        method_name: String,
    }

    fn next_batch(state: &mut State) -> Option<Batch> {
        if !state.data.notifications.is_queue_empty() {
            let mut notifications = Vec::with_capacity(min(
                state.data.notifications.queue_len(),
                MAX_NOTIFICATIONS_PER_BATCH,
            ));

            while let Some(notification) = state.data.notifications.dequeue() {
                notifications.push(notification);
                if notifications.len() == MAX_NOTIFICATIONS_PER_BATCH {
                    break;
                }
            }

            Some(Batch {
                notifications,
                method_name: state.data.notification_method_name.clone(),
            })
        } else {
            None
        }
    }

    async fn push_batch(batch: Batch) {
        let method_name = &batch.method_name;
        let futures: Vec<_> = batch
            .notifications
            .into_iter()
            .map(|n| push(n, method_name))
            .collect();

        futures::future::join_all(futures).await;
    }

    async fn push(notification: Notification, method_name: &str) {
        let response: CallResult<()> =
            ic_cdk::call(notification.canister_id, method_name, (notification.args,)).await;

        match response {
            Ok(_) => mutate_state(|state| state.data.notifications.mark_sent()),
            Err(_error) => {
                // TODO handle this
            }
        }
    }
}

use crate::model::ledger_sync_state::TryStartSyncResult;
use crate::model::notifications_queue::Notification;
use crate::{mutate_state, NotifyTransactionArgs, State, Subscriptions};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::heartbeat;
use ic_ledger_types::{
    AccountIdentifier, ArchivedBlocksRange, Block, BlockIndex, GetBlocksArgs, GetBlocksResult,
    Operation,
};
use itertools::Itertools;
use std::collections::HashSet;
use tracing::error;
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
        ledger: CanisterId,
        block_index_synced_up_to: Option<BlockIndex>,
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
                if let TryStartSyncResult::Success(block_index_synced_up_to) =
                    t.ledger_sync_state_mut().try_start(now)
                {
                    Some(TokenToSync {
                        token_symbol: t.token_symbol().to_string(),
                        ledger: t.ledger(),
                        block_index_synced_up_to,
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
        if let Some(from_block_index) = token_to_sync.block_index_synced_up_to.map(|i| i + 1) {
            match blocks_since(token_to_sync.ledger, from_block_index, 1000).await {
                Ok(blocks) if !blocks.is_empty() => mutate_state(|state| {
                    new_block_index_synced_up_to = Some(from_block_index - 1 + blocks.len() as u64);
                    enqueue_notifications(
                        &token_to_sync.token_symbol,
                        token_to_sync.ledger,
                        blocks,
                        from_block_index,
                        state,
                    );
                }),
                Ok(_) => {}
                Err(error) => error!(?error, "Failed to get blocks from ledger"),
            }
        } else {
            match chain_length(token_to_sync.ledger).await {
                Ok(chain_length) => {
                    new_block_index_synced_up_to = Some(chain_length);
                }
                Err(error) => {
                    error!(?error, "Failed to get chain length from ledger")
                }
            }
        }

        mutate_state(|state| {
            mark_sync_complete(
                &token_to_sync.token_symbol,
                new_block_index_synced_up_to,
                state,
            )
        });
    }

    async fn chain_length(ledger_canister_id: CanisterId) -> CallResult<BlockIndex> {
        ic_ledger_types::query_blocks(
            ledger_canister_id,
            GetBlocksArgs {
                start: 0,
                length: 0,
            },
        )
        .await
        .map(|res| res.chain_length)
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
        state: &mut State,
    ) {
        if let Some(token_data) = state.data.tokens.get_mut(token_symbol) {
            let ledger_sync_state = token_data.ledger_sync_state_mut();

            if let Some(block_index) = new_block_index_synced_up_to {
                ledger_sync_state.set_synced_up_to(block_index);
            }
            ledger_sync_state.mark_sync_complete();
        }
    }

    fn enqueue_notifications(
        token_symbol: &str,
        ledger: CanisterId,
        blocks: Vec<Block>,
        from_block_index: BlockIndex,
        state: &mut State,
    ) {
        if let Some(subscriptions) = state
            .data
            .tokens
            .get(token_symbol)
            .map(|t| t.subscriptions())
        {
            for (block_index, block) in blocks
                .into_iter()
                .enumerate()
                .map(|(index, block)| ((index as u64) + from_block_index, block))
            {
                let account_identifiers = extract_account_identifiers(&block.transaction.operation);
                let canisters_to_notify =
                    extract_canisters_to_notify(&account_identifiers, subscriptions);

                for canister_id in canisters_to_notify {
                    state.data.notifications_queue.add(Notification {
                        canister_id,
                        args: NotifyTransactionArgs {
                            token_symbol: token_symbol.to_string(),
                            ledger,
                            block_index,
                            block: block.clone(),
                        },
                    })
                }
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
        if !state.data.notifications_queue.is_empty() {
            let mut notifications = Vec::new();
            while let Some(notification) = state.data.notifications_queue.take() {
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

        if let Err(error) = response {
            // TODO handle this
        }
    }
}

use crate::{mutate_state, read_state, State};
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::heartbeat;
use ic_ledger_types::{
    AccountIdentifier, ArchivedBlocksRange, Block, BlockIndex, GetBlocksArgs, GetBlocksResult,
    Operation,
};
use tracing::error;
use types::CanisterId;

#[heartbeat]
fn heartbeat() {
    sync_ledger_transactions::run();
}

mod sync_ledger_transactions {
    use super::*;
    use crate::model::notifications_queue::Notification;
    use itertools::Itertools;
    use std::collections::HashSet;

    pub fn run() {
        if let Some(block_index_synced_up_to) = mutate_state(|state| {
            let now = state.env.now();
            state.data.ledger_sync_state.try_start(now)
        }) {
            ic_cdk::spawn(sync_transactions(block_index_synced_up_to + 1));
        }
    }

    async fn sync_transactions(from_block_index: BlockIndex) {
        let ledger_canister_id = read_state(|state| state.data.ledger_canister_id);

        match blocks_since(ledger_canister_id, from_block_index, 1000).await {
            Ok(blocks) => mutate_state(|state| process_blocks(blocks, from_block_index, state)),
            Err(error) => error!(?error, "Failed to get blocks from ledger"),
        }

        mutate_state(|state| state.data.ledger_sync_state.mark_sync_complete());
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

    fn process_blocks(blocks: Vec<Block>, from_block_index: BlockIndex, state: &mut State) {
        if blocks.is_empty() {
            return;
        }

        let last_block_index = from_block_index + blocks.len() as u64;

        for (block_index, block) in blocks
            .into_iter()
            .enumerate()
            .map(|(index, block)| ((index as u64) + from_block_index, block))
        {
            let account_identifiers = extract_account_identifiers(&block.transaction.operation);
            let canisters_to_notify = extract_canisters_to_notify(&account_identifiers, state);

            for canister_id in canisters_to_notify {
                state.data.notifications_queue.add(Notification {
                    canister_id,
                    block_index,
                    block: block.clone(),
                })
            }
        }

        state
            .data
            .ledger_sync_state
            .set_synced_up_to(last_block_index);
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
        state: &State,
    ) -> HashSet<CanisterId> {
        HashSet::from_iter(
            account_identifiers
                .iter()
                .filter_map(|a| state.data.subscriptions.get(a))
                .flatten()
                .copied(),
        )
    }
}

use crate::guards::caller_is_admin;
use crate::{mutate_state, read_state, State, TokenData};
use canister_tracing_macros::trace;
use ic_cdk::api::call::CallResult;
use ic_cdk_macros::update;
use ic_ledger_types::{BlockIndex, GetBlocksArgs};
use std::collections::hash_map::Entry::Vacant;
use transaction_notifier::add_token::{Response::*, *};
use types::CanisterId;

#[update(guard = "caller_is_admin")]
#[trace]
async fn add_token(args: Args) -> Response {
    if read_state(|state| {
        state
            .data
            .tokens
            .values()
            .any(|t| t.ledger_canister_id() == args.ledger_canister_id)
    }) {
        AlreadyAdded
    } else {
        let token_symbol_future = token_symbol(args.ledger_canister_id);
        let block_index_future = latest_block_index(args.ledger_canister_id);

        let (token_symbol_res, latest_block_index_res) =
            futures::future::join(token_symbol_future, block_index_future).await;

        match (token_symbol_res, latest_block_index_res) {
            (Ok(token_symbol), Ok(latest_block_index)) => mutate_state(|state| {
                add_token_impl(
                    token_symbol,
                    args.ledger_canister_id,
                    latest_block_index,
                    state,
                )
            }),
            (Err(err), _) | (_, Err(err)) => LedgerError(format!("{:?}", err)),
        }
    }
}

fn add_token_impl(
    token_symbol: String,
    ledger_canister_id: CanisterId,
    latest_block_index: BlockIndex,
    state: &mut State,
) -> Response {
    match state.data.tokens.entry(token_symbol.clone()) {
        Vacant(e) => {
            e.insert(TokenData::new(
                token_symbol,
                ledger_canister_id,
                latest_block_index,
            ));
            Success
        }
        _ => AlreadyAdded,
    }
}

async fn token_symbol(ledger_canister_id: CanisterId) -> CallResult<String> {
    ic_ledger_types::token_symbol(ledger_canister_id)
        .await
        .map(|res| res.symbol)
}

async fn latest_block_index(ledger_canister_id: CanisterId) -> CallResult<BlockIndex> {
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

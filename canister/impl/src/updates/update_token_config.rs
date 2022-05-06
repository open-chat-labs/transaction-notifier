use crate::guards::caller_is_admin;
use crate::{mutate_state, State};
use canister_tracing_macros::trace;
use ic_cdk_macros::update;
use transaction_notifier::update_token_config::{Response::*, *};

#[update(guard = "caller_is_admin")]
#[trace]
fn update_token_config(args: Args) -> Response {
    mutate_state(|state| update_token_config_impl(args, state))
}

fn update_token_config_impl(args: Args, state: &mut State) -> Response {
    if let Some(token) = state.data.tokens.get_mut(&args.token_symbol) {
        let ledger_sync_state = token.ledger_sync_state_mut();
        if let Some(enabled) = args.sync_enabled {
            ledger_sync_state.set_enabled(enabled);
        }
        if let Some(block_index) = args.sync_from_block_index {
            ledger_sync_state.set_synced_up_to(block_index, None);
            ledger_sync_state.incr_version();
        }
        Success
    } else {
        TokenNotFound
    }
}

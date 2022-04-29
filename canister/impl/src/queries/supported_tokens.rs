use crate::{read_state, State};
use canister_tracing_macros::trace;
use ic_cdk_macros::update;
use transaction_notifier::supported_tokens::{Response::*, *};

#[update]
#[trace]
fn supported_tokens(_args: Args) -> Response {
    read_state(supported_tokens_impl)
}

fn supported_tokens_impl(state: &State) -> Response {
    let tokens = state.data.tokens.keys().cloned().collect();
    Success(tokens)
}

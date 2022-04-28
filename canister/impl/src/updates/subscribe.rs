use crate::{mutate_state, State};
use canister_tracing_macros::trace;
use ic_cdk_macros::update;
use transaction_notifier::subscribe::{Response::*, *};

#[update]
#[trace]
fn subscribe(args: Args) -> Response {
    mutate_state(|state| subscribe_impl(args, state))
}

fn subscribe_impl(args: Args, state: &mut State) -> Response {
    for subscription in args.subscriptions {
        if let Some(token_data) = state.data.tokens.get_mut(&subscription.token_symbol) {
            token_data
                .subscriptions_mut()
                .add(subscription.account_identifier, subscription.canister_ids);
        } else {
            panic!("Token not supported: {}", subscription.token_symbol);
        }
    }
    Success
}

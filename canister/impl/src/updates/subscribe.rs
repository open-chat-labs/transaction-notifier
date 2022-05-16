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
        state
            .data
            .subscriptions
            .add(subscription.account_identifier, subscription.canister_ids);
    }
    Success
}

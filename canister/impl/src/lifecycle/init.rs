use crate::env::CanisterEnv;
use crate::lifecycle::{init_logger, init_state};
use crate::Data;
use canister_tracing_macros::trace;
use ic_cdk_macros::init;
use tracing::info;
use transaction_notifier::init::Args;

#[init]
#[trace]
fn init(args: Args) {
    ic_cdk::setup();
    init_logger(args.test_mode);

    let env = Box::new(CanisterEnv::default());

    let data = Data::new(
        args.token_symbol,
        args.ledger_canister_id,
        args.admins.into_iter().collect(),
        args.notification_method_name
            .unwrap_or("notify_transaction".to_string()),
        args.test_mode,
    );

    init_state(env, data, args.wasm_version);

    info!(version = %args.wasm_version, "Initialization complete");
}

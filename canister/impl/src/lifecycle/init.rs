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
        args.admins.into_iter().collect(),
        args.notification_method_name
            .unwrap_or_else(|| "notify_transaction".to_string()),
        args.test_mode,
    );

    let version = args.wasm_version;

    init_state(env, data, version);

    info!(%version, "Initialization complete");
}

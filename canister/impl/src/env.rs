use candid::Principal;
use types::{CanisterId, Cycles, TimestampMillis};

const NANOS_PER_SECOND: u64 = 1_000_000_000;

pub trait Environment {
    fn now(&self) -> TimestampMillis;
    fn caller(&self) -> Principal;
    fn canister_id(&self) -> CanisterId;
    fn cycles_balance(&self) -> Cycles;
}

#[derive(Default)]
pub struct CanisterEnv {}

impl Environment for CanisterEnv {
    fn now(&self) -> TimestampMillis {
        ic_cdk::api::time() / NANOS_PER_SECOND
    }

    fn caller(&self) -> Principal {
        ic_cdk::caller()
    }

    fn canister_id(&self) -> CanisterId {
        ic_cdk::id()
    }

    fn cycles_balance(&self) -> Cycles {
        ic_cdk::api::canister_balance().into()
    }
}

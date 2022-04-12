use crate::env::Environment;
use candid::Principal;
use canister_logger::LogMessagesWrapper;
use canister_state_macros::canister_state;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashSet;
use types::{CanisterId, Timestamped, Version};

mod env;
mod lifecycle;

thread_local! {
    static LOG_MESSAGES: RefCell<LogMessagesWrapper> = RefCell::default();
    static WASM_VERSION: RefCell<Timestamped<Version>> = RefCell::default();
}

canister_state!(State);

struct State {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl State {
    pub fn new(env: Box<dyn Environment>, data: Data) -> State {
        State { env, data }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    ledger_canister_id: CanisterId,
    admins: HashSet<Principal>,
    test_mode: bool,
}

impl Data {
    pub fn new(
        ledger_canister_id: CanisterId,
        admins: HashSet<Principal>,
        test_mode: bool,
    ) -> Data {
        Data {
            ledger_canister_id,
            admins,
            test_mode,
        }
    }
}

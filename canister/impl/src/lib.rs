use crate::env::Environment;
use crate::model::ledger_sync_state::LedgerSyncState;
use crate::model::notifications_queue::NotificationsQueue;
use crate::model::subscriptions::Subscriptions;
use crate::model::token_data::TokenData;
use candid::{CandidType, Principal};
use canister_logger::LogMessagesWrapper;
use canister_state_macros::canister_state;
use ic_ledger_types::{Block, BlockIndex};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use types::{CanisterId, Timestamped, Version};

mod env;
mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;

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
    admins: HashSet<Principal>,
    notification_method_name: String,
    tokens: HashMap<String, TokenData>,
    notifications_queue: NotificationsQueue,
    test_mode: bool,
}

impl Data {
    pub fn new(
        admins: HashSet<Principal>,
        notification_method_name: String,
        test_mode: bool,
    ) -> Data {
        Data {
            admins,
            notification_method_name,
            tokens: HashMap::default(),
            notifications_queue: NotificationsQueue::default(),
            test_mode,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NotifyTransactionArgs {
    pub token_symbol: String,
    pub ledger_canister_id: CanisterId,
    pub block_index: BlockIndex,
    pub block: Block,
}

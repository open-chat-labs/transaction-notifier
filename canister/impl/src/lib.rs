use crate::env::Environment;
use crate::model::ledger_sync_state::LedgerSyncState;
use crate::model::notifications::Notifications;
use crate::model::subscriptions::Subscriptions;
use crate::model::token_data::TokenData;
use candid::{CandidType, Principal};
use canister_logger::LogMessagesWrapper;
use canister_state_macros::canister_state;
use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use types::{CanisterId, Cycles, TimestampMillis, Timestamped, Version};

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

    pub fn metrics(&self) -> Metrics {
        Metrics {
            now: self.env.now(),
            memory_used: 0,
            cycles_balance: self.env.cycles_balance(),
            wasm_version: WASM_VERSION.with(|v| **v.borrow()),
            tokens: self.data.tokens.values().map(|t| t.metrics()).collect(),
            subscriptions: self.data.subscriptions.len() as u64,
            notifications_sent: self.data.notifications.total_sent(),
            notifications_queued: self.data.notifications.queue_len().try_into().unwrap(),
            test_mode: self.data.test_mode,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    admins: HashSet<Principal>,
    notification_method_name: String,
    tokens: HashMap<String, TokenData>,
    subscriptions: Subscriptions,
    notifications: Notifications,
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
            subscriptions: Subscriptions::default(),
            notifications: Notifications::default(),
            test_mode,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Metrics {
    pub now: TimestampMillis,
    pub memory_used: u64,
    pub cycles_balance: Cycles,
    pub wasm_version: Version,
    pub tokens: Vec<TokenMetrics>,
    pub subscriptions: u64,
    pub notifications_sent: u64,
    pub notifications_queued: u64,
    pub test_mode: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TokenMetrics {
    pub token_symbol: String,
    pub ledger_canister_id: CanisterId,
    pub sync_enabled: bool,
    pub synced_up_to: Option<BlockIndex>,
    pub last_sync_started_at: TimestampMillis,
    pub last_successful_sync: TimestampMillis,
    pub last_failed_sync: TimestampMillis,
}

use ic_ledger_types::AccountIdentifier;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use types::CanisterId;

#[derive(Serialize, Deserialize, Default)]
pub struct Subscriptions {
    subscription: HashMap<AccountIdentifier, HashSet<CanisterId>>,
}

impl Subscriptions {
    pub fn add(&mut self, account_identifier: AccountIdentifier, canister_ids: Vec<CanisterId>) {
        let canisters_subscribed = self.subscription.entry(account_identifier).or_default();
        for canister_id in canister_ids {
            canisters_subscribed.insert(canister_id);
        }
    }
}

use candid::CandidType;
use ic_ledger_types::AccountIdentifier;
use serde::Deserialize;
use types::CanisterId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub subscriptions: Vec<Subscription>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct Subscription {
    pub account_identifier: AccountIdentifier,
    pub canister_ids: Vec<CanisterId>,
}

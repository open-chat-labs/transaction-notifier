use crate::{LedgerSyncState, Subscriptions};
use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    token_symbol: String,
    ledger_canister_id: CanisterId,
    ledger_sync_state: LedgerSyncState,
    subscriptions: Subscriptions,
}

impl TokenData {
    pub fn new(
        token_symbol: String,
        ledger_canister_id: CanisterId,
        latest_block_index: BlockIndex,
    ) -> TokenData {
        TokenData {
            token_symbol,
            ledger_canister_id,
            ledger_sync_state: LedgerSyncState::new(latest_block_index),
            subscriptions: Subscriptions::default(),
        }
    }

    pub fn token_symbol(&self) -> &str {
        &self.token_symbol
    }

    pub fn ledger_canister_id(&self) -> CanisterId {
        self.ledger_canister_id
    }

    pub fn ledger_sync_state_mut(&mut self) -> &mut LedgerSyncState {
        &mut self.ledger_sync_state
    }

    pub fn subscriptions(&self) -> &Subscriptions {
        &self.subscriptions
    }

    pub fn subscriptions_mut(&mut self) -> &mut Subscriptions {
        &mut self.subscriptions
    }
}

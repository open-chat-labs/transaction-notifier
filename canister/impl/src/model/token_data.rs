use crate::{LedgerSyncState, Subscriptions};
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    token_symbol: String,
    ledger: CanisterId,
    ledger_sync_state: LedgerSyncState,
    subscriptions: Subscriptions,
}

impl TokenData {
    pub fn new(token_symbol: String, ledger: CanisterId) -> TokenData {
        TokenData {
            token_symbol,
            ledger,
            ledger_sync_state: LedgerSyncState::default(),
            subscriptions: Subscriptions::default(),
        }
    }

    pub fn token_symbol(&self) -> &str {
        &self.token_symbol
    }

    pub fn ledger(&self) -> CanisterId {
        self.ledger
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

use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

#[derive(Serialize, Deserialize, Default)]
pub struct LedgerSyncState {
    in_progress: bool,
    synced_up_to: Option<BlockIndex>,
    last_sync_started_at: TimestampMillis,
}

impl LedgerSyncState {
    pub fn try_start(&mut self, now: TimestampMillis) -> TryStartSyncResult {
        if !self.in_progress {
            self.in_progress = true;
            self.last_sync_started_at = now;
            TryStartSyncResult::Success(self.synced_up_to)
        } else {
            TryStartSyncResult::AlreadyInProgress
        }
    }

    pub fn mark_sync_complete(&mut self) {
        self.in_progress = false;
    }

    pub fn synced_up_to(&self) -> Option<BlockIndex> {
        self.synced_up_to
    }

    pub fn set_synced_up_to(&mut self, block_index: BlockIndex) {
        self.synced_up_to = Some(block_index);
    }

    pub fn last_sync_started_at(&self) -> TimestampMillis {
        self.last_sync_started_at
    }
}

pub enum TryStartSyncResult {
    Success(Option<BlockIndex>),
    AlreadyInProgress,
}

use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

pub type Version = u32;

#[derive(Serialize, Deserialize)]
pub struct LedgerSyncState {
    enabled: bool,
    in_progress: bool,
    synced_up_to: BlockIndex,
    last_sync_started_at: TimestampMillis,
    last_successful_sync: TimestampMillis,
    last_failed_sync: TimestampMillis,
    version: Version,
}

impl LedgerSyncState {
    pub fn new(latest_block_index: BlockIndex) -> LedgerSyncState {
        LedgerSyncState {
            enabled: false,
            in_progress: false,
            synced_up_to: latest_block_index,
            last_sync_started_at: 0,
            last_successful_sync: 0,
            last_failed_sync: 0,
            version: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn try_start(&mut self, now: TimestampMillis) -> TryStartSyncResult {
        if !self.enabled {
            TryStartSyncResult::Disabled
        } else if !self.in_progress {
            self.in_progress = true;
            self.last_sync_started_at = now;
            TryStartSyncResult::Success(self.synced_up_to, self.version)
        } else {
            TryStartSyncResult::AlreadyInProgress
        }
    }

    pub fn mark_sync_complete(&mut self, success: bool, now: TimestampMillis) {
        self.in_progress = false;

        if success {
            self.last_successful_sync = now;
        } else {
            self.last_failed_sync = now;
        }
    }

    pub fn synced_up_to(&self) -> BlockIndex {
        self.synced_up_to
    }

    pub fn set_synced_up_to(&mut self, block_index: BlockIndex, version_check: Option<Version>) {
        if version_check.map_or(true, |v| v == self.version) {
            self.synced_up_to = block_index;
        }
    }

    pub fn last_sync_started_at(&self) -> TimestampMillis {
        self.last_sync_started_at
    }

    pub fn last_successful_sync(&self) -> TimestampMillis {
        self.last_successful_sync
    }

    pub fn last_failed_sync(&self) -> TimestampMillis {
        self.last_failed_sync
    }

    pub fn incr_version(&mut self) {
        self.version += 1;
    }
}

pub enum TryStartSyncResult {
    Success(BlockIndex, Version),
    AlreadyInProgress,
    Disabled,
}

use candid::Principal;

mod timestamped;
mod version;

pub use timestamped::*;
pub use version::*;

pub type CanisterId = Principal;
pub type Cycles = u128;
pub type TimestampMillis = u64;

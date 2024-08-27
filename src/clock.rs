use derive_more::{From, Into};
use napi::bindgen_prelude::*;
use solana_program::clock::Clock as ClockOriginal;

/// A representation of network time.
///
/// All members of `Clock` start from 0 upon network boot.
#[derive(From, Into, Default)]
#[napi]
pub struct Clock(ClockOriginal);

impl AsRef<ClockOriginal> for Clock {
  fn as_ref(&self) -> &ClockOriginal {
    &self.0
  }
}

#[napi]
impl Clock {
  /// @param slot - The current Slot.
  /// @param epochStartTimestamp - The timestamp of the first `Slot` in this `Epoch`.
  /// @param epoch - The current epoch.
  /// @param leaderScheduleEpoch - The future Epoch for which the leader schedule has most recently been calculated.
  /// @param unixTimestamp - The approximate real world time of the current slot.
  #[napi(constructor)]
  pub fn new(
    slot: BigInt,
    epoch_start_timestamp: BigInt,
    epoch: BigInt,
    leader_schedule_epoch: BigInt,
    unix_timestamp: BigInt,
  ) -> Self {
    ClockOriginal {
      slot: slot.get_u64().1,
      epoch_start_timestamp: epoch_start_timestamp.get_i64().0,
      epoch: epoch.get_u64().1,
      leader_schedule_epoch: leader_schedule_epoch.get_u64().1,
      unix_timestamp: unix_timestamp.get_i64().0,
    }
    .into()
  }

  /// The current Slot.
  #[napi(getter)]
  pub fn slot(&self) -> u64 {
    self.0.slot
  }

  /// The current epoch.
  #[napi(getter)]
  pub fn epoch(&self) -> u64 {
    self.0.epoch
  }

  /// The timestamp of the first `Slot` in this `Epoch`.
  #[napi(getter)]
  pub fn epoch_start_timestamp(&self) -> BigInt {
    BigInt::from(self.0.epoch_start_timestamp)
  }

  /// The future Epoch for which the leader schedule has most recently been calculated.
  #[napi(getter)]
  pub fn leader_schedule_epoch(&self) -> u64 {
    self.0.leader_schedule_epoch
  }

  /// The approximate real world time of the current slot.
  #[napi(getter)]
  pub fn unix_timestamp(&self) -> BigInt {
    BigInt::from(self.0.unix_timestamp)
  }
}

use derive_more::{From, Into};
use solana_program::epoch_schedule::EpochSchedule as EpochScheduleOriginal;

#[derive(From, Into)]
#[napi]
pub struct EpochSchedule(EpochScheduleOriginal);

#[napi]
impl EpochSchedule {
  #[napi(getter)]
  pub fn slots_per_epoch(&self) -> u64 {
    self.0.slots_per_epoch
  }
  #[napi(getter)]
  pub fn leader_schedule_slot_offset(&self) -> u64 {
    self.0.leader_schedule_slot_offset
  }
  #[napi(getter)]
  pub fn warmup(&self) -> bool {
    self.0.warmup
  }
  #[napi(getter)]
  pub fn first_normal_epoch(&self) -> u64 {
    self.0.first_normal_epoch
  }
  #[napi(getter)]
  pub fn first_normal_slot(&self) -> u64 {
    self.0.first_normal_slot
  }
}

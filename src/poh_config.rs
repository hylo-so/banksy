use derive_more::{From, Into};
use solana_sdk::poh_config::PohConfig as PohConfigOriginal;

#[derive(From, Into)]
#[napi]
pub struct PohConfig(PohConfigOriginal);

#[napi]
impl PohConfig {
  #[napi(getter)]
  pub fn target_tick_duration(&self) -> u128 {
    self.0.target_tick_duration.as_micros()
  }
  #[napi(getter)]
  pub fn target_tick_count(&self) -> Option<u64> {
    self.0.target_tick_count
  }
  #[napi(getter)]
  pub fn hashes_per_tick(&self) -> Option<u64> {
    self.0.hashes_per_tick
  }
}

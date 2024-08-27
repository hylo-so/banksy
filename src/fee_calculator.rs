use derive_more::{From, Into};
use solana_program::fee_calculator::FeeRateGovernor as FeeRateGovernorOriginal;

#[derive(From, Into)]
#[napi]
pub struct FeeRateGovernor(FeeRateGovernorOriginal);

#[napi]
impl FeeRateGovernor {
  #[napi(getter)]
  pub fn lamports_per_signature(&self) -> u64 {
    self.0.lamports_per_signature
  }
  #[napi(getter)]
  pub fn target_lamports_per_signature(&self) -> u64 {
    self.0.target_lamports_per_signature
  }
  #[napi(getter)]
  pub fn target_signatures_per_slot(&self) -> u64 {
    self.0.target_signatures_per_slot
  }
  #[napi(getter)]
  pub fn min_lamports_per_signature(&self) -> u64 {
    self.0.min_lamports_per_signature
  }
  #[napi(getter)]
  pub fn max_lamports_per_signature(&self) -> u64 {
    self.0.max_lamports_per_signature
  }
  #[napi(getter)]
  pub fn burn_percent(&self) -> u8 {
    self.0.burn_percent
  }
}

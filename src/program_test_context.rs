use crate::{
  convert_to_pubkey, Account, AccountOriginal, AccountSharedData, BanksClient, Clock,
  GenesisConfig, Rent,
};
use derive_more::{From, Into};
use napi::bindgen_prelude::*;
use solana_test_framework::ProgramTestContext as ProgramTestContextOriginal;

#[derive(From, Into)]
#[napi]
pub struct ProgramTestContext(ProgramTestContextOriginal);

#[napi]
impl ProgramTestContext {
  #[napi(getter)]
  pub fn banks_client(&self) -> BanksClient {
    self.0.banks_client.clone().into()
  }

  #[napi(getter)]
  pub fn payer(&self) -> Uint8Array {
    self.0.payer.to_bytes().into()
  }

  #[napi(getter)]
  pub fn last_blockhash(&self) -> String {
    self.0.last_blockhash.to_string()
  }

  #[napi(getter)]
  pub fn genesis_config(&self) -> GenesisConfig {
    self.0.genesis_config().clone().into()
  }

  #[napi]
  pub fn set_account(&mut self, address: Uint8Array, account: &Account) {
    let acc_original: &AccountOriginal = account.as_ref();
    let acc_shared = AccountSharedData::from(acc_original.clone());
    self.0.set_account(&convert_to_pubkey(address), &acc_shared);
  }

  #[napi]
  pub fn set_clock(&mut self, clock: &Clock) {
    self.0.set_sysvar(clock.as_ref())
  }

  #[napi]
  pub fn set_rent(&mut self, rent: &Rent) {
    self.0.set_sysvar(rent.as_ref())
  }

  #[napi]
  pub fn warp_to_slot(&mut self, warp_slot: BigInt) -> Result<()> {
    self.0.warp_to_slot(warp_slot.get_u64().1).map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to warp to slot {:?}: {e}", warp_slot),
      )
    })
  }

  #[napi]
  pub fn warp_to_epoch(&mut self, warp_epoch: BigInt) -> Result<()> {
    self.0.warp_to_epoch(warp_epoch.get_u64().1).map_err(|e| {
      Error::new(
        Status::GenericFailure,
        format!("Failed to warp to slot {:?}: {e}", warp_epoch),
      )
    })
  }
}

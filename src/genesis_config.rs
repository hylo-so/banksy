use crate::{
  Account, AddressAndAccount, EpochSchedule, FeeRateGovernor, Inflation,
  NativeInstructionProcessor, PohConfig, Rent,
};
use derive_more::{From, Into};
use napi::bindgen_prelude::*;
use solana_sdk::genesis_config::{ClusterType, GenesisConfig as GenesisConfigOriginal};

#[derive(From, Into)]
#[napi]
pub struct GenesisConfig(GenesisConfigOriginal);

#[napi]
impl GenesisConfig {
  #[napi(getter)]
  pub fn creation_time(&self) -> i64 {
    self.0.creation_time
  }
  #[napi(getter)]
  pub fn accounts(&self) -> Vec<AddressAndAccount> {
    self
      .0
      .accounts
      .clone()
      .into_iter()
      .map(|(pk, acc)| AddressAndAccount::new(Uint8Array::from(pk.to_bytes()), Account::from(acc)))
      .collect()
  }
  #[napi(getter)]
  pub fn native_instruction_processors(&self) -> Vec<NativeInstructionProcessor> {
    self
      .0
      .native_instruction_processors
      .clone()
      .into_iter()
      .map(|tup| NativeInstructionProcessor {
        string_val: tup.0,
        pubkey_val: Uint8Array::from(tup.1.to_bytes()),
      })
      .collect()
  }
  #[napi(getter)]
  pub fn rewards_pools(&self) -> Vec<AddressAndAccount> {
    self
      .0
      .rewards_pools
      .clone()
      .into_iter()
      .map(|(pk, acc)| AddressAndAccount::new(Uint8Array::from(pk.to_bytes()), Account::from(acc)))
      .collect()
  }

  #[napi(getter)]
  pub fn ticks_per_slot(&self) -> u64 {
    self.0.ticks_per_slot
  }
  #[napi(getter)]
  pub fn poh_config(&self) -> PohConfig {
    self.0.poh_config.clone().into()
  }
  #[napi(getter)]
  pub fn fee_rate_governor(&self) -> FeeRateGovernor {
    self.0.fee_rate_governor.clone().into()
  }
  #[napi(getter)]
  pub fn rent(&self) -> Rent {
    self.0.rent.clone().into()
  }
  #[napi(getter)]
  pub fn inflation(&self) -> Inflation {
    self.0.inflation.into()
  }
  #[napi(getter)]
  pub fn epoch_schedule(&self) -> EpochSchedule {
    self.0.epoch_schedule.clone().into()
  }
  #[napi(getter)]
  pub fn cluster_type(&self) -> String {
    match self.0.cluster_type {
      ClusterType::Development => "development",
      ClusterType::Devnet => "devnet",
      ClusterType::MainnetBeta => "mainnet-beta",
      ClusterType::Testnet => "testnet",
    }
    .to_string()
  }
}

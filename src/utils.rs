use crate::Account;
use derive_more::{From, Into};
use napi::bindgen_prelude::*;
use solana_program::pubkey::Pubkey;

pub fn convert_to_pubkey(address: Uint8Array) -> Pubkey {
  Pubkey::try_from(address.as_ref()).unwrap()
}

pub fn convert_to_option_pubkey(address: Option<Uint8Array>) -> Option<Pubkey> {
  address.and_then(|a| Pubkey::try_from(a.as_ref()).ok())
}

pub fn convert_to_option_u64(amount: Option<BigInt>) -> Option<u64> {
  amount.map(|b| b.get_u64().1)
}

#[derive(Debug, Clone)]
#[napi]
pub struct BlockhashRes {
  pub blockhash: String,
  pub last_valid_block_height: BigInt,
}

#[derive(From, Into)]
#[napi]
pub struct AddressAndAccount {
  pub address: Uint8Array,
  account: Account,
}

impl AddressAndAccount {
  pub fn new(address: Uint8Array, account: Account) -> Self {
    AddressAndAccount { address, account }
  }
}

#[napi]
impl AddressAndAccount {
  #[napi(getter)]
  pub fn account(&self) -> Account {
    self.account.clone()
  }
}

#[derive(From, Into)]
#[napi]
pub struct NativeInstructionProcessor {
  pub string_val: String,
  pub pubkey_val: Uint8Array,
}

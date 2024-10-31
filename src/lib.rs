#![allow(clippy::missing_safety_doc)]

use napi::bindgen_prelude::*;
use solana_program::system_program::ID as SYSTEM_PROGRAM;
use solana_sdk::account::{Account as AccountOriginal, AccountSharedData};
use solana_test_framework::ProgramTest;

#[macro_use]
extern crate napi_derive;

mod account;
mod banks_client;
mod call_msg;
mod clock;
mod commitment;
mod epoch_schedule;
mod fee_calculator;
mod genesis_config;
mod inflation;
mod poh_config;
mod program_test_context;
mod rent;
mod transaction;
mod utils;

pub use account::*;
pub use banks_client::*;
pub use call_msg::*;
pub use clock::*;
pub use commitment::*;
pub use epoch_schedule::*;
pub use fee_calculator::*;
pub use genesis_config::*;
pub use inflation::*;
pub use poh_config::*;
pub use program_test_context::*;
pub use rent::*;
pub use transaction::*;
pub use utils::*;

#[napi]
pub struct Banksy {
  call_stack: Vec<CallMsg>,
}

#[napi]
impl Banksy {
  #[napi(constructor)]
  pub fn new() -> Self {
    Banksy { call_stack: vec![] }
  }

  #[napi]
  pub async fn start(
    &self,
    compute_max_units: Option<BigInt>,
    transaction_account_lock_limit: Option<BigInt>,
  ) -> ProgramTestContext {
    let mut pt = ProgramTest::default();
    if let Some(cmu) = compute_max_units {
      pt.set_compute_max_units(cmu.get_u64().1);
    }
    if let Some(limit) = transaction_account_lock_limit {
      pt.set_transaction_account_lock_limit(usize::try_from(limit.get_u64().1).unwrap());
    }
    CallMsg::process(&mut pt, &self.call_stack);
    pt.start_with_context().await.into()
  }

  #[napi]
  pub fn add_program(
    &mut self,
    name: String,
    program_id: Uint8Array,
    program_authority: Uint8Array,
  ) {
    self.call_stack.push(CallMsg::AddProgram {
      name,
      program_id: convert_to_pubkey(program_id),
      program_authority: convert_to_pubkey(program_authority),
    })
  }

  #[napi]
  pub fn add_upgradable_program(
    &mut self,
    name: String,
    program_id: Uint8Array,
    program_authority: Uint8Array,
    program_data: Uint8Array,
    upgrade_slot: BigInt,
  ) {
    self.call_stack.push(CallMsg::AddUpgradeableProgram {
      name: name.to_string(),
      program_id: convert_to_pubkey(program_id),
      program_authority: convert_to_pubkey(program_authority),
      program_data: convert_to_pubkey(program_data),
      upgrade_slot: upgrade_slot.get_u64().1,
    })
  }

  #[napi]
  pub fn add_account(&mut self, address: Uint8Array, account: &Account) {
    self.call_stack.push(CallMsg::AddAccount {
      address: convert_to_pubkey(address),
      account: account.clone().into(),
    })
  }

  #[napi]
  pub fn add_account_with_lamports(
    &mut self,
    address: Uint8Array,
    owner: Uint8Array,
    lamports: BigInt,
  ) {
    self.call_stack.push(CallMsg::AddAccountWithLamports {
      address: convert_to_pubkey(address),
      owner: convert_to_pubkey(owner),
      lamports: lamports.get_u64().1,
    })
  }

  #[napi]
  pub fn airdrop(&mut self, address: Uint8Array, lamports: BigInt) {
    self.call_stack.push(CallMsg::AddAccountWithLamports {
      address: convert_to_pubkey(address),
      owner: SYSTEM_PROGRAM,
      lamports: lamports.get_u64().1,
    })
  }

  #[napi]
  pub fn add_token_mint(
    &mut self,
    address: Uint8Array,
    supply: BigInt,
    decimals: u8,
    mint_authority: Option<Uint8Array>,
    freeze_authority: Option<Uint8Array>,
  ) {
    self.call_stack.push(CallMsg::AddTokenMint {
      address: convert_to_pubkey(address),
      supply: supply.get_u64().1,
      decimals,
      mint_authority: convert_to_option_pubkey(mint_authority),
      freeze_authority: convert_to_option_pubkey(freeze_authority),
    })
  }

  #[napi]
  pub fn add_token_account(
    &mut self,
    address: Uint8Array,
    mint: Uint8Array,
    owner: Uint8Array,
    amount: BigInt,
    delegated_amount: BigInt,
    delegate: Option<Uint8Array>,
    is_native: Option<BigInt>,
    close_authority: Option<Uint8Array>,
  ) {
    self.call_stack.push(CallMsg::AddTokenAccount {
      address: convert_to_pubkey(address),
      mint: convert_to_pubkey(mint),
      owner: convert_to_pubkey(owner),
      amount: amount.get_u64().1,
      delegated_amount: delegated_amount.get_u64().1,
      delegate: convert_to_option_pubkey(delegate),
      is_native: convert_to_option_u64(is_native),
      close_authority: convert_to_option_pubkey(close_authority),
    })
  }
}

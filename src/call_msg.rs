use crate::AccountOriginal;
use solana_program::pubkey::Pubkey;
use solana_test_framework::{ProgramTest, ProgramTestExtension};

pub enum CallMsg {
  AddProgram {
    name: String,
    program_id: Pubkey,
    program_authority: Pubkey,
  },
  AddUpgradeableProgram {
    name: String,
    program_id: Pubkey,
    program_authority: Pubkey,
    program_data: Pubkey,
  },
  AddAccount {
    address: Pubkey,
    account: AccountOriginal,
  },
  AddAccountWithLamports {
    address: Pubkey,
    owner: Pubkey,
    lamports: u64,
  },
  AddTokenMint {
    address: Pubkey,
    mint_authority: Option<Pubkey>,
    supply: u64,
    decimals: u8,
    freeze_authority: Option<Pubkey>,
  },
  AddTokenAccount {
    address: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Option<Pubkey>,
    is_native: Option<u64>,
    delegated_amount: u64,
    close_authority: Option<Pubkey>,
  },
}

impl CallMsg {
  pub fn process(pt: &mut ProgramTest, call_stack: &Vec<CallMsg>) {
    for msg in call_stack {
      match msg {
        CallMsg::AddAccount { address, account } => {
          pt.add_account(address.clone(), account.clone().into())
        }
        CallMsg::AddProgram {
          name,
          program_id,
          program_authority,
        } => pt.add_bpf_program(
          name.as_str(),
          program_id.clone(),
          Some(program_authority.clone()),
          None,
        ),
        CallMsg::AddUpgradeableProgram {
          name,
          program_id,
          program_authority,
          program_data,
        } => pt.add_bpf_program_with_program_data(
          name.as_str(),
          program_id.clone(),
          Some(program_authority.clone()),
          program_data.clone(),
          None,
        ),
        CallMsg::AddAccountWithLamports {
          address,
          owner,
          lamports,
        } => pt.add_account_with_lamports(address.clone(), owner.clone(), lamports.clone()),
        CallMsg::AddTokenMint {
          address,
          mint_authority,
          supply,
          decimals,
          freeze_authority,
        } => pt.add_token_mint(
          address.clone(),
          mint_authority.clone(),
          supply.clone(),
          decimals.clone(),
          freeze_authority.clone(),
        ),
        CallMsg::AddTokenAccount {
          address,
          mint,
          owner,
          amount,
          delegate,
          is_native,
          delegated_amount,
          close_authority,
        } => pt.add_token_account(
          address.clone(),
          mint.clone(),
          owner.clone(),
          amount.clone(),
          delegate.clone(),
          is_native.clone(),
          delegated_amount.clone(),
          close_authority.clone(),
        ),
      }
    }
  }
}

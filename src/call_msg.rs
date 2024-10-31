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
    upgrade_slot: u64,
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
        CallMsg::AddAccount { address, account } => pt.add_account(*address, account.clone()),
        CallMsg::AddProgram {
          name,
          program_id,
          program_authority,
        } => pt.add_bpf_program(name.as_str(), *program_id, Some(*program_authority), None),
        CallMsg::AddUpgradeableProgram {
          name,
          program_id,
          program_authority,
          program_data,
          upgrade_slot,
        } => pt.add_bpf_program_with_program_data(
          name.as_str(),
          *program_id,
          Some(*program_authority),
          *program_data,
          *upgrade_slot,
          None,
        ),
        CallMsg::AddAccountWithLamports {
          address,
          owner,
          lamports,
        } => pt.add_account_with_lamports(*address, *owner, *lamports),
        CallMsg::AddTokenMint {
          address,
          mint_authority,
          supply,
          decimals,
          freeze_authority,
        } => pt.add_token_mint(
          *address,
          *mint_authority,
          *supply,
          *decimals,
          *freeze_authority,
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
          *address,
          *mint,
          *owner,
          *amount,
          *delegate,
          *is_native,
          *delegated_amount,
          *close_authority,
        ),
      }
    }
  }
}

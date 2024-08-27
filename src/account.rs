use derive_more::{From, Into};
use napi::bindgen_prelude::*;

use solana_program::pubkey::Pubkey;

use solana_sdk::account::Account as AccountOriginal;

#[derive(Debug, From, Into, Clone)]
#[napi]
pub struct Account(AccountOriginal);

impl AsRef<AccountOriginal> for Account {
  fn as_ref(&self) -> &AccountOriginal {
    &self.0
  }
}

#[napi]
impl Account {
  #[napi(constructor)]
  pub fn new(
    lamports: BigInt,
    data: Uint8Array,
    owner: Uint8Array,
    executable: bool,
    rent_epoch: BigInt,
  ) -> Self {
    AccountOriginal {
      lamports: lamports.get_u64().1,
      data: data.to_vec(),
      owner: Pubkey::try_from(owner.as_ref()).unwrap(),
      executable,
      rent_epoch: rent_epoch.get_u64().1,
    }
    .into()
  }

  #[napi(getter)]
  pub fn lamports(&self) -> u64 {
    self.0.lamports
  }

  #[napi(getter)]
  pub fn data(&self) -> Uint8Array {
    Uint8Array::new(self.0.data.clone())
  }

  #[napi(getter)]
  pub fn owner(&self) -> Uint8Array {
    Uint8Array::new(self.0.owner.to_bytes().to_vec())
  }

  #[napi(getter)]
  pub fn executable(&self) -> bool {
    self.0.executable
  }

  #[napi(getter)]
  pub fn rent_epoch(&self) -> u64 {
    self.0.rent_epoch
  }
}

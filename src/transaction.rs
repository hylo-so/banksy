use derive_more::{From, Into};
use napi::bindgen_prelude::*;
use solana_banks_interface::{
  BanksTransactionResultWithMetadata, BanksTransactionResultWithSimulation, TransactionMetadata,
};
use solana_sdk::{
  transaction::TransactionError,
  transaction_context::TransactionReturnData as TransactionReturnDataOriginal,
};
use solana_transaction_status::{
  TransactionConfirmationStatus, TransactionStatus as TransactionStatusOriginal,
};

#[derive(Debug, From, Into, Clone)]
#[napi]
pub struct TransactionReturnData(TransactionReturnDataOriginal);

#[napi]
impl TransactionReturnData {
  #[napi(getter)]
  pub fn program_id(&self) -> Uint8Array {
    Uint8Array::new(self.0.program_id.to_bytes().to_vec())
  }

  #[napi(getter)]
  pub fn data(&self) -> Uint8Array {
    Uint8Array::new(self.0.data.clone())
  }
}

#[derive(Debug, From, Into, Clone)]
#[napi]
pub struct BanksTransactionMeta(TransactionMetadata);

#[napi]
impl BanksTransactionMeta {
  #[napi(getter)]
  pub fn log_messages(&self) -> Vec<String> {
    self.0.log_messages.clone()
  }

  #[napi(getter)]
  pub fn return_data(&self) -> Option<TransactionReturnData> {
    self.0.return_data.clone().map(Into::into)
  }

  #[napi(getter)]
  pub fn compute_units_consumed(&self) -> u64 {
    self.0.compute_units_consumed
  }
}

#[derive(Debug, From, Into, Clone)]
#[napi]
pub struct TransactionStatus(TransactionStatusOriginal);

#[napi]
impl TransactionStatus {
  pub fn new(
    slot: u64,
    confirmations: Option<usize>,
    err: Option<TransactionError>,
    confirmation_status: Option<TransactionConfirmationStatus>,
  ) -> Self {
    TransactionStatusOriginal {
      slot,
      confirmations,
      status: Ok(()),
      err: err.map(Into::into),
      confirmation_status: confirmation_status.map(Into::into),
    }
    .into()
  }

  #[napi(getter)]
  pub fn slot(&self) -> u64 {
    self.0.slot
  }

  #[napi(getter)]
  pub fn confirmations(&self) -> Option<usize> {
    self.0.confirmations
  }

  #[napi(getter)]
  pub fn err(&self) -> Option<String> {
    self.0.err.clone().map(|x| x.to_string())
  }

  #[napi(getter)]
  pub fn confirmation_status(&self) -> Option<String> {
    self
      .0
      .confirmation_status
      .clone()
      .map(|x| serde_json::to_string(&x).unwrap())
  }
}

#[derive(Debug, From, Into, Clone)]
#[napi]
pub struct BanksTransactionResultWithMeta(BanksTransactionResultWithMetadata);

#[napi]
impl BanksTransactionResultWithMeta {
  #[napi(getter)]
  pub fn result(&self) -> Option<String> {
    match self.0.result.clone() {
      Ok(()) => None,
      Err(x) => Some(x.to_string()),
    }
  }

  #[napi(getter)]
  pub fn meta(&self) -> Option<BanksTransactionMeta> {
    self.0.metadata.clone().map(Into::into)
  }
}

impl From<BanksTransactionResultWithSimulation> for BanksTransactionResultWithMeta {
  fn from(r: BanksTransactionResultWithSimulation) -> Self {
    BanksTransactionResultWithMetadata {
      result: match r.result {
        None => Ok(()),
        Some(x) => x,
      },
      metadata: r.simulation_details.map(|d| TransactionMetadata {
        log_messages: d.logs,
        compute_units_consumed: d.units_consumed,
        return_data: d.return_data,
      }),
    }
    .into()
  }
}

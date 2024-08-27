use crate::{
  convert_to_pubkey, Account, BanksTransactionMeta, BanksTransactionResultWithMeta, BlockhashRes,
  Clock, CommitmentLevel, Rent, TransactionStatus,
};
use bincode::deserialize;
use derive_more::{From, Into};
use napi::bindgen_prelude::*;
use solana_banks_client::TransactionStatus as TransactionStatusBanks;
use solana_banks_interface::TransactionConfirmationStatus as TransactionConfirmationStatusBanks;
use solana_program::{clock::Clock as ClockOriginal, message::Message};
use solana_sdk::{
  signature::Signature,
  transaction::{Transaction, VersionedTransaction},
};
use solana_test_framework::{BanksClient as BanksClientOriginal, BanksClientError};
use solana_transaction_status::TransactionConfirmationStatus;
use tarpc::context::current;

#[derive(From, Into)]
#[napi]
pub struct BanksClient(BanksClientOriginal);

fn to_js_error(e: BanksClientError, msg: &str) -> Error {
  Error::new(Status::GenericFailure, format!("{msg}: {e}"))
}

fn confirmation_status_from_banks(
  s: TransactionConfirmationStatusBanks,
) -> TransactionConfirmationStatus {
  match s {
    TransactionConfirmationStatusBanks::Processed => TransactionConfirmationStatus::Processed,
    TransactionConfirmationStatusBanks::Confirmed => TransactionConfirmationStatus::Confirmed,
    TransactionConfirmationStatusBanks::Finalized => TransactionConfirmationStatus::Finalized,
  }
}

fn transaction_status_from_banks(t: TransactionStatusBanks) -> TransactionStatus {
  TransactionStatus::new(
    t.slot,
    t.confirmations,
    t.err.map(Into::into),
    t.confirmation_status.map(confirmation_status_from_banks),
  )
}

#[napi]
impl BanksClient {
  #[napi]
  pub async unsafe fn get_account(
    &mut self,
    address: Uint8Array,
    commitment: Option<CommitmentLevel>,
  ) -> Result<Option<Account>> {
    let pk = convert_to_pubkey(address);
    let res = self
      .0
      .get_account_with_commitment(pk, commitment.unwrap_or_default().into())
      .await;
    match res {
      Ok(x) => Ok(x.map(Into::into)),
      Err(e) => Err(to_js_error(e, "Failed to load account")),
    }
  }

  #[napi]
  pub async unsafe fn send_legacy_transaction(&mut self, tx_bytes: Uint8Array) -> Result<()> {
    let tx: Transaction = deserialize(&tx_bytes).unwrap();
    let res = self.0.send_transaction(tx).await;
    res.map_err(|e| to_js_error(e, "Failed to process transaction"))
  }

  #[napi]
  pub async unsafe fn send_versioned_transaction(&mut self, tx_bytes: Uint8Array) -> Result<()> {
    let tx: VersionedTransaction = deserialize(&tx_bytes).unwrap();
    let res = self.0.send_transaction(tx).await;
    res.map_err(|e| to_js_error(e, "Failed to process transaction"))
  }

  #[napi]
  pub async unsafe fn process_legacy_transaction(
    &mut self,
    tx_bytes: Uint8Array,
  ) -> Result<BanksTransactionMeta> {
    let tx: Transaction = deserialize(&tx_bytes).unwrap();
    let res = self.0.process_transaction_with_metadata(tx).await;
    match res {
      Ok(r) => match r.result {
        Err(e) => Err(to_js_error(
          BanksClientError::TransactionError(e),
          "Failed to process transaction",
        )),
        Ok(()) => Ok(BanksTransactionMeta::from(r.metadata.unwrap())),
      },
      Err(e) => Err(to_js_error(e, "Failed to process transaction")),
    }
  }

  #[napi]
  pub async unsafe fn process_versioned_transaction(
    &mut self,
    tx_bytes: Uint8Array,
  ) -> Result<BanksTransactionMeta> {
    let tx: VersionedTransaction = deserialize(&tx_bytes).unwrap();
    let res = self.0.process_transaction_with_metadata(tx).await;
    match res {
      Ok(r) => match r.result {
        Err(e) => Err(to_js_error(
          BanksClientError::TransactionError(e),
          "Failed to process transaction",
        )),
        Ok(()) => Ok(BanksTransactionMeta::from(r.metadata.unwrap())),
      },
      Err(e) => Err(to_js_error(e, "Failed to process transaction")),
    }
  }

  #[napi]
  pub async unsafe fn try_process_legacy_transaction(
    &mut self,
    tx_bytes: Uint8Array,
  ) -> Result<BanksTransactionResultWithMeta> {
    let tx: Transaction = deserialize(&tx_bytes).unwrap();
    let res = self.0.process_transaction_with_metadata(tx).await;
    match res {
      Ok(r) => Ok(r.into()),
      Err(e) => Err(to_js_error(e, "Failed to process transaction")),
    }
  }

  #[napi]
  pub async unsafe fn try_process_versioned_transaction(
    &mut self,
    tx_bytes: Uint8Array,
  ) -> Result<BanksTransactionResultWithMeta> {
    let tx: VersionedTransaction = deserialize(&tx_bytes).unwrap();
    let res = self.0.process_transaction_with_metadata(tx).await;
    match res {
      Ok(r) => Ok(r.into()),
      Err(e) => Err(to_js_error(e, "Failed to process transaction")),
    }
  }

  #[napi]
  pub async unsafe fn simulate_legacy_transaction(
    &mut self,
    tx_bytes: Uint8Array,
    commitment: Option<CommitmentLevel>,
  ) -> Result<BanksTransactionResultWithMeta> {
    let tx: Transaction = deserialize(&tx_bytes).unwrap();
    let res = self
      .0
      .simulate_transaction_with_commitment(tx, commitment.unwrap_or_default().into())
      .await;
    match res {
      Ok(x) => Ok(x.into()),
      Err(e) => Err(to_js_error(e, "Failed to simulate transaction")),
    }
  }

  #[napi]
  pub async unsafe fn simulate_versioned_transaction(
    &mut self,
    tx_bytes: Uint8Array,
    commitment: Option<CommitmentLevel>,
  ) -> Result<BanksTransactionResultWithMeta> {
    let tx: VersionedTransaction = deserialize(&tx_bytes).unwrap();
    let res = self
      .0
      .simulate_transaction_with_commitment(tx, commitment.unwrap_or_default().into())
      .await;
    match res {
      Ok(x) => Ok(x.into()),
      Err(e) => Err(to_js_error(e, "Failed to simulate transaction")),
    }
  }

  #[napi]
  pub async unsafe fn get_transaction_status(
    &mut self,
    signature: Uint8Array,
  ) -> Result<Option<TransactionStatus>> {
    let sig = Signature::try_from(signature.as_ref()).unwrap();
    let res = self.0.get_transaction_status(sig).await;
    match res {
      Ok(x) => Ok(x.map(transaction_status_from_banks)),
      Err(e) => Err(to_js_error(e, "Failed to get transaction status.")),
    }
  }

  #[napi]
  pub async unsafe fn get_transaction_statuses(
    &mut self,
    signatures: Vec<Uint8Array>,
  ) -> Result<Vec<Option<TransactionStatus>>> {
    let sigs: Vec<Signature> = signatures
      .into_iter()
      .map(|s| Signature::try_from(s.as_ref()).unwrap())
      .collect();
    let res = self.0.get_transaction_statuses(sigs).await;
    match res {
      Ok(x) => Ok(
        x.iter()
          .map(|o| o.clone().map(transaction_status_from_banks))
          .collect(),
      ),
      Err(e) => Err(to_js_error(e, "Failed to get transaction statuses.")),
    }
  }

  #[napi]
  pub async unsafe fn get_slot(&mut self, commitment: Option<CommitmentLevel>) -> Result<u64> {
    let res = self
      .0
      .get_slot_with_context(current(), commitment.unwrap_or_default().into())
      .await;
    res.map_err(|e| to_js_error(e, "Failed to get slot"))
  }

  #[napi]
  pub async unsafe fn get_block_height(
    &mut self,
    commitment: Option<CommitmentLevel>,
  ) -> Result<u64> {
    let res = self
      .0
      .get_block_height_with_context(current(), commitment.unwrap_or_default().into())
      .await;
    res.map_err(|e| to_js_error(e, "Failed to get block height"))
  }

  #[napi]
  pub async unsafe fn get_rent(&mut self) -> Result<Rent> {
    let res = self.0.get_rent().await;
    match res {
      Ok(x) => Ok(x.into()),
      Err(e) => Err(to_js_error(e, "Failed to get rent")),
    }
  }

  #[napi]
  pub async unsafe fn get_clock(&mut self) -> Result<Clock> {
    let res = self.0.get_sysvar::<ClockOriginal>().await;
    match res {
      Ok(x) => Ok(x.into()),
      Err(e) => Err(to_js_error(e, "Failed to get clock")),
    }
  }

  #[napi]
  pub async unsafe fn get_balance(
    &mut self,
    address: Uint8Array,
    commitment: Option<CommitmentLevel>,
  ) -> Result<u64> {
    let res = self
      .0
      .get_balance_with_commitment(
        convert_to_pubkey(address),
        commitment.unwrap_or_default().into(),
      )
      .await;
    res.map_err(|e| to_js_error(e, "Failed to get balance"))
  }

  #[napi]
  pub async unsafe fn get_latest_blockhash(
    &mut self,
    commitment: Option<CommitmentLevel>,
  ) -> Result<Option<BlockhashRes>> {
    let res = self
      .0
      .get_latest_blockhash_with_commitment(commitment.unwrap_or_default().into())
      .await;
    match res {
      Ok(x) => Ok(x.map(|tup| BlockhashRes {
        blockhash: tup.0.to_string(),
        last_valid_block_height: BigInt::from(tup.1),
      })),
      Err(e) => Err(to_js_error(e, "Failed to get latest blockhash")),
    }
  }

  #[napi]
  pub async unsafe fn get_fee_for_message(
    &mut self,
    message_bytes: Uint8Array,
    commitment: Option<CommitmentLevel>,
  ) -> Result<Option<u64>> {
    let msg: Message = bincode::deserialize(message_bytes.as_ref()).unwrap();
    let res = self
      .0
      .get_fee_for_message_with_commitment_and_context(
        current(),
        msg,
        commitment.unwrap_or_default().into(),
      )
      .await;
    res.map_err(|e| to_js_error(e, "Failed to get latest blockhash"))
  }
}

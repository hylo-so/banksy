use solana_sdk::commitment_config::CommitmentLevel as CommitmentLevelOriginal;

#[napi]
pub enum CommitmentLevel {
  Processed,
  Confirmed,
  Finalized,
}

impl Default for CommitmentLevel {
  fn default() -> Self {
    Self::Confirmed
  }
}

impl From<CommitmentLevel> for CommitmentLevelOriginal {
  fn from(value: CommitmentLevel) -> Self {
    match value {
      CommitmentLevel::Processed => Self::Processed,
      CommitmentLevel::Confirmed => Self::Confirmed,
      CommitmentLevel::Finalized => Self::Finalized,
    }
  }
}

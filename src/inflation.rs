use derive_more::{From, Into};
use solana_sdk::inflation::Inflation as InflationOriginal;

#[derive(From, Into)]
#[napi]
pub struct Inflation(InflationOriginal);

#[napi]
impl Inflation {
  #[napi(getter)]
  pub fn initial(&self) -> f64 {
    self.0.initial
  }
  #[napi(getter)]
  pub fn terminal(&self) -> f64 {
    self.0.terminal
  }
  #[napi(getter)]
  pub fn taper(&self) -> f64 {
    self.0.taper
  }
  #[napi(getter)]
  pub fn foundation(&self) -> f64 {
    self.0.foundation
  }
  #[napi(getter)]
  pub fn foundation_term(&self) -> f64 {
    self.0.foundation_term
  }
}

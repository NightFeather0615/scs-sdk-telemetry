#[cfg(feature = "json")]
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "json", serde(rename_all(serialize = "snake_case")))]
pub enum JobMarket {
  #[default]
  NoValue,
  CargoMarket,
  QuickJob,
  FreightMarket,
  ExternalContracts,
  ExternalMarket
}

impl JobMarket {
  pub fn from_str(value: &str) -> JobMarket {
    match value {
      "cargo_market" => Self::CargoMarket,
      "quick_job" => Self::QuickJob,
      "freight_market" => Self::FreightMarket,
      "external_contracts" => Self::ExternalContracts,
      "external_market" => Self::ExternalMarket,
      _ => Self::NoValue
    }
  }
}

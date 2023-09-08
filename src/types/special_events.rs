#[derive(Debug, Clone, Default)]
pub struct SpecialEvents {
  pub on_job: bool,
  pub job_finished: bool,
  pub job_cancelled: bool,
  pub job_delivered: bool,
  pub fined: bool,
  pub tollgate: bool,
  pub ferry: bool,
  pub train: bool,
  pub refuel: bool,
  pub refuel_payed: bool,
}

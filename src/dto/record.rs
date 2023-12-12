use uuid::Uuid;

pub struct IdRecord {
  pub id: Uuid,
}
pub struct TotalRecord {
  pub total: Option<i64>,
}

pub struct ExistRecord {
  pub exist: Option<bool>,
}

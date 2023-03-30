use error::AppResponseError;
use model::response::MessageResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppResponseResult<T = MessageResponse> {
  Ok(T),
  Err(AppResponseError),
}

impl<T> AppResponseResult<T> {
  pub fn is_ok(&self) -> bool {
    matches!(*self, Self::Ok(_))
  }

  pub fn is_err(&self) -> bool {
    matches!(*self, Self::Err(_))
  }

  pub fn unwrap(self) -> T {
    match self {
      Self::Ok(t) => t,
      Self::Err(e) => panic!("called `AppResult::unwrap()` on an `Err` value {:?}", &e),
    }
  }
}

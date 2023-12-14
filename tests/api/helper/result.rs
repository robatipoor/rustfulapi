use rustfulapi::{dto::MessageResponse, error::AppResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppResponseResult<T = MessageResponse> {
  Ok(T),
  Err(AppResponseError),
}

#[allow(dead_code)]
impl<T> AppResponseResult<T> {
  pub fn is_ok(&self) -> bool {
    matches!(*self, Self::Ok(_))
  }

  pub fn is_err(&self) -> bool {
    matches!(*self, Self::Err(_))
  }
}

#[macro_export]
macro_rules! unwrap {
  ($result:expr) => {
    match $result {
      $crate::helper::result::AppResponseResult::Ok(resp) => resp,
      $crate::helper::result::AppResponseResult::Err(err) => {
        panic!("Called `common::unwrap!()` on an `Err` value {err:?}.")
      }
    }
  };
}

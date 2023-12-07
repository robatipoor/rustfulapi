#[macro_export]
macro_rules! retry {
  ($func:expr) => {{
    let mut remaining_attempts = $crate::constant::MAX_RETRY;
    let mut delay = $crate::constant::MINIMUM_DELAY_TIME;
    loop {
      remaining_attempts -= 1;
      let result = $func().await;
      if result.is_ok() {
        break result;
      } else if remaining_attempts == 0 {
        tracing::warn!("Maximum number of attempts exceeded.");
        break result;
      }
      tokio::time::sleep(delay).await;
      delay += $crate::constant::MINIMUM_DELAY_TIME;
    }
  }};
  ($func:expr,$predicate:expr) => {{
    let mut remaining_attempts = $crate::constant::MAX_RETRY;
    let mut delay = $crate::constant::MINIMUM_DELAY_TIME;
    loop {
      remaining_attempts -= 1;
      let result = $func().await;
      if $predicate(&result) {
        break result;
      } else if remaining_attempts == 0 {
        tracing::warn!("Maximum number of attempts exceeded.");
        break result;
      }
      tokio::time::sleep(delay).await;
      delay += $crate::constant::MINIMUM_DELAY_TIME;
    }
  }};
}

#[macro_export]
macro_rules! assert_err {
    ($result:expr $(, $closure:expr )?) => {
      #[allow(clippy::redundant_closure_call)]
      {
        assert!(
          matches!($result, $crate::helper::result::AppResponseResult::Err(ref _e) $( if $closure(_e) )?),
          "Match failed: {:?}",$result,
        )
    }};
}

#[macro_export]
macro_rules! assert_ok {

    ($result:expr $(, $closure:expr )?) => {
      #[allow(clippy::redundant_closure_call)]
      {
        assert!(
          matches!($result, $crate::helper::result::AppResponseResult::Ok(ref _d) $( if $closure(_d) )?),
          "Match failed: {:?}",$result,
        )
    }};
}

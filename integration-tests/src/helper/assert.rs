#[macro_export]
macro_rules! assert_ok {
  ($result:expr) => {
    assert!(
      matches!($result, $crate::helper::result::AppResponseResult::Ok(_)),
      "match failed: {:?}",
      $result,
    )
  };
}

#[macro_export]
macro_rules! assert_err {
    ($result:expr $(, $closure:expr )?) => {
        assert!(
          matches!($result,$crate::helper::result::AppResponseResult::Err(ref _e) $( if $closure(_e) )?),
          "match failed: {:?}",$result,
        )
    };
}

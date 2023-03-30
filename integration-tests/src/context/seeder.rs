use crate::helper::user::TestUser;

use super::app::AppTestContext;
use entity::role::RoleUser;
use std::collections::HashMap;
use test_context::AsyncTestContext;

pub struct SeedDbTestContext {
  pub app: AppTestContext,
  pub users: HashMap<RoleUser, TestUser>,
}

#[async_trait::async_trait]
impl AsyncTestContext for SeedDbTestContext {
  async fn setup() -> Self {
    let app = AppTestContext::setup().await;
    let users = TestUser::create_users(&app.state.postgres).await.unwrap();
    Self { app, users }
  }

  async fn teardown(mut self) {
    self.app.teardown().await
  }
}

use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

use error::AppResult;

pub use constant::root_dir;

pub async fn save_file(file_path: &Path, content: &[u8]) -> AppResult<()> {
  if let Some(parent_dir) = file_path.parent() {
    fs::create_dir_all(&parent_dir).await?;
  }
  let mut file = fs::File::create(&file_path).await?;
  file.write_all(content).await?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use std::path::PathBuf;

  use super::save_file;
  use constant::{APP_IMAGE, IMAGES_PATH};

  use once_cell::sync::Lazy;
  use test_context::{test_context, AsyncTestContext};
  use tokio::fs;
  use uuid::Uuid;

  pub use super::*;
  #[allow(dead_code)]
  struct FileTestContext {
    content: Vec<u8>,
    path: PathBuf,
  }

  #[async_trait::async_trait]
  impl AsyncTestContext for FileTestContext {
    async fn setup() -> Self {
      let image = Lazy::force(&APP_IMAGE);
      let content = fs::read(image).await.unwrap();
      let path = Lazy::force(&IMAGES_PATH).join("tmp").join(format!(
        "{}_{}",
        Uuid::new_v4(),
        image.file_name().unwrap().to_str().unwrap()
      ));
      Self { content, path }
    }

    async fn teardown(self) {
      fs::remove_file(self.path).await.unwrap();
    }
  }

  #[test_context(FileTestContext)]
  #[tokio::test]
  pub async fn test_save_file(ctx: &mut FileTestContext) {
    save_file(&ctx.path, &ctx.content).await.unwrap();
    let result = fs::read(&ctx.path).await;
    assert!(result.is_ok())
  }
}

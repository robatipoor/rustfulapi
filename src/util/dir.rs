use std::path::PathBuf;

pub fn get_project_root() -> std::io::Result<PathBuf> {
  if let Some(root) = get_cargo_project_root()? {
    Ok(root)
  } else {
    Ok(std::env::current_dir()?)
  }
}

pub fn get_cargo_project_root() -> std::io::Result<Option<PathBuf>> {
  let current_path = std::env::current_dir()?;

  for ancestor in current_path.ancestors() {
    for dir in std::fs::read_dir(ancestor)? {
      let dir = dir?;
      if dir.file_name() == *"Cargo.lock" {
        return Ok(Some(ancestor.to_path_buf()));
      }
    }
  }
  Ok(None)
}

#[cfg(test)]
mod tests {
  use super::get_cargo_project_root;

  #[test]
  fn test_get_cargo_project_root() {
    let root = get_cargo_project_root().unwrap().unwrap();
    assert_eq!(root.file_name().unwrap().to_str().unwrap(), "rustfulapi");
  }
}

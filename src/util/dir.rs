use std::path::Path;

pub fn root_dir<P: AsRef<Path> + ?Sized>(path: &P) -> std::io::Result<std::path::PathBuf> {
  Ok(
    project_root::get_project_root()
      .or_else(|_| std::env::current_dir())?
      .join(path),
  )
}

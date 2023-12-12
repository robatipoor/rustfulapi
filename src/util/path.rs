use std::path::{Component, Path, PathBuf};

pub fn normalize<P: AsRef<Path>>(path: &P) -> PathBuf {
  let mut components = path.as_ref().components().peekable();
  let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
    components.next();
    PathBuf::from(c.as_os_str())
  } else {
    PathBuf::new()
  };

  for component in components {
    match component {
      Component::Prefix(..) => unreachable!(),
      Component::CurDir | Component::RootDir => {}
      Component::ParentDir => {
        ret.pop();
      }
      Component::Normal(c) => {
        ret.push(c);
      }
    }
  }
  ret
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use crate::util::path::normalize;

  #[test]
  fn test_normalize() {
    assert_eq!(normalize(&Path::new("./test")), Path::new("test"));
    assert_eq!(normalize(&Path::new(".//test")), Path::new("test"));
    assert_eq!(normalize(&Path::new("test")), Path::new("test"));
    assert_eq!(
      normalize(&Path::new("./a/b/c/../test.txt")),
      Path::new("a/b/test.txt")
    );
    assert_eq!(
      normalize(&Path::new("../a/b/c/test.txt")),
      Path::new("a/b/c/test.txt")
    );
    assert_eq!(normalize(&Path::new("/a/b/c")), Path::new("a/b/c"));
  }
}

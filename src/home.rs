use std::env;
use std::path::PathBuf;

pub fn home_dir() -> Option<PathBuf> {
  home_dir_inner()
}

#[cfg(windows)]
fn home_dir_inner() -> Option<PathBuf> {
  env::var_os("USERPROFILE")
    .filter(|s| !s.is_empty())
    .map(PathBuf::from)
}

#[cfg(any(unix, target_os = "redox"))]
fn home_dir_inner() -> Option<PathBuf> {
  #[allow(deprecated)]
  env::home_dir()
}

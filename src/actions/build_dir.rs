use crate::Actions;
use std::io::{stdout, Write};

const BUILD_DIR: &str = "rust-tools-target";

impl Actions {
  pub(crate) fn build_dir(&self) -> crate::Result<()> {
    let mut stdout = stdout();
    stdout.write_all(BUILD_DIR.as_bytes())?;
    stdout.flush()?;
    Ok(())
  }
}

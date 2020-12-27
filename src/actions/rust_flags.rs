use crate::{handle_cmd_output, Actions};
use std::{
  io::{stdout, Write},
  process::Command,
};

impl Actions {
  pub fn rust_flags(&self) -> crate::Result<()> {
    handle_cmd_output(
      Command::new("echo").args(&["-e", r#"\e[0;33m***** Applying Rust flags *****\e[0m\n"#]),
    )?;
    let mut iter = self.params.rust_flags.iter();
    let mut stdout = stdout();
    if let Some(first) = iter.next() {
      stdout.write_all(first.as_bytes())?;
    }
    for element in iter {
      stdout.write_all(br#" "#)?;
      stdout.write_all(element.as_bytes())?;
    }
    stdout.flush()?;
    Ok(())
  }
}

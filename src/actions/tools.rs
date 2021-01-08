use crate::{handle_cmd_output, Actions};
use std::process::Command;

impl Actions {
  pub(crate) fn clippy(&self) -> crate::Result<()> {
    handle_cmd_output(
      Command::new("echo").args(&["-e", r#"\e[0;33m***** Running clippy *****\e[0m\n"#]),
    )?;
    let mut cmd = Command::new("cargo");
    handle_cmd_output(
      cmd.args(&["clippy", "--all-features", "--"]).args(&self.params.clippy_flags),
    )?;
    Ok(())
  }

  pub(crate) fn rustfmt(&self) -> crate::Result<()> {
    handle_cmd_output(
      Command::new("echo").args(&["-e", r#"\e[0;33m***** Running rustfmt *****\e[0m\n"#]),
    )?;
    handle_cmd_output(Command::new("cargo").args(&["fmt", "--all", "--", "--check"]))?;
    Ok(())
  }
}

use crate::{handle_cmd_output, Actions};
use std::process::Command;

impl Actions {
  pub fn clippy(&self) -> crate::Result<()> {
    handle_cmd_output(
      Command::new("echo").args(&["-e", r#"\e[0;33m***** Running clippy *****\e[0m\n"#]),
    )?;
    let mut cmd = Command::new("cargo");
    cmd.args(&["clippy", "--all-features", "--"]);
    cmd.args(&self.params.clippy_flags);
    handle_cmd_output(&mut cmd)?;
    Ok(())
  }

  pub fn rustfmt(&self) -> crate::Result<()> {
    handle_cmd_output(
      Command::new("echo").args(&["-e", r#"\e[0;33m***** Running rustfmt *****\e[0m\n"#]),
    )?;
    handle_cmd_output(Command::new("cargo").args(&["fmt", "--all", "--", "--check"]))?;
    Ok(())
  }
}

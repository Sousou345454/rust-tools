use crate::{handle_cmd_output, manage_flags};
use std::process::Command;

const CLIPPY_FLAGS: [&str; 4] = [
    "-Dclippy::restriction",
    "-Dwarnings",
    "-Aclippy::implicit_return",
    "-Aclippy::missing_docs_in_private_items",
];

pub fn clippy(to_add_string: String, to_subtract_string: String) -> crate::Result<()> {
    handle_cmd_output(
        Command::new("echo").args(&["-e", r#"\e[0;33m***** Running clippy *****\e[0m\n"#]),
    )?;
    let mut cmd = Command::new("cargo");
    cmd.args(&["clippy", "--all-features", "--"]);
    cmd.args(manage_flags(
        &CLIPPY_FLAGS,
        &to_add_string,
        &to_subtract_string,
    ));
    handle_cmd_output(&mut cmd)?;
    Ok(())
}

pub fn rustfmt() -> crate::Result<()> {
    handle_cmd_output(
        Command::new("echo").args(&["-e", r#"\e[0;33m***** Running rustfmt *****\e[0m\n"#]),
    )?;
    handle_cmd_output(Command::new("cargo").args(&["fmt", "--all", "--", "--check"]))?;
    Ok(())
}

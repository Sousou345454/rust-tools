use crate::handle_cmd_output;
use std::process::Command;

macro_rules! create_fns {
    (
        $cargo_action:literal, $info:literal, $generic_fn:ident, $with_features_fn:ident
    ) => {
        pub fn $generic_fn(package: String) -> crate::Result<()> {
            handle_cmd_output(Command::new("echo").arg("-e").arg(format!(
                concat!(
                    r#"\e[0;33m***** "#,
                    $info,
                    r#" "{}" without features *****\e[0m\n"#
                ),
                package
            )))?;
            handle_cmd_output(
                Command::new("cargo")
                    .arg($cargo_action)
                    .arg("--manifest-path")
                    .arg(&format!("{}/Cargo.toml", package))
                    .arg("--no-default-features"),
            )?;

            handle_cmd_output(Command::new("echo").arg("-e").arg(format!(
                concat!(
                    r#"\e[0;33m***** "#,
                    $info,
                    r#" "{}" with all features *****\e[0m\n"#
                ),
                package
            )))?;
            handle_cmd_output(
                Command::new("cargo")
                    .arg($cargo_action)
                    .arg("--all-features")
                    .arg("--manifest-path")
                    .arg(&format!("{}/Cargo.toml", package)),
            )?;
            Ok(())
        }

        pub fn $with_features_fn(package: String, features: String) -> crate::Result<()> {
            handle_cmd_output(Command::new("echo").arg("-e").arg(format!(
                concat!(
                    r#"\e[0;33m***** "#,
                    $info,
                    r#" "{}" with features "{}" *****\e[0m\n"#
                ),
                package, features
            )))?;
            handle_cmd_output(
                Command::new("cargo")
                    .arg($cargo_action)
                    .arg("--features")
                    .arg(features)
                    .arg("--manifest-path")
                    .arg(&format!("{}/Cargo.toml", package))
                    .arg("--no-default-features"),
            )?;
            Ok(())
        }
    };
}

create_fns!("check", "Checking", check_generic, check_with_features);
create_fns!("test", "Testing", test_generic, test_with_features);

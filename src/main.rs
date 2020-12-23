//! Rust tools

mod action;
mod cargo;
mod error;
mod rust_flags;
mod tools;

use action::Action;
use error::Error;
use std::{
    env::{args, Args},
    io::{stderr, stdout, BufRead, BufReader, Write},
    process::{Command, Stdio},
};

type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut args = args();
    let req = |args: &mut Args| args.next().ok_or(Error::WrongNumberOfVars);
    let opt = |args: &mut Args| args.next().unwrap_or_default();
    let _ = req(&mut args)?;
    match req(&mut args)?.parse()? {
        Action::Clippy => tools::clippy(opt(&mut args), opt(&mut args))?,
        Action::CheckWithFeatures => cargo::check_with_features(req(&mut args)?, opt(&mut args))?,
        Action::CheckGeneric => cargo::check_generic(req(&mut args)?)?,
        Action::RustFlags => rust_flags::rust_flags(opt(&mut args), opt(&mut args))?,
        Action::Rustfmt => tools::rustfmt()?,
        Action::TestGeneric => cargo::test_generic(req(&mut args)?)?,
        Action::TestWithFeatures => cargo::test_with_features(req(&mut args)?, opt(&mut args))?,
    };
    Ok(())
}

fn handle_cmd_output(mut cmd: &mut Command) -> Result<()> {
    cmd = cmd.stdin(Stdio::piped()).stdout(Stdio::piped());
    let child = cmd.spawn()?;
    let mut buffer = String::new();
    macro_rules! write {
        ($inner:expr, $output:expr) => {
            let mut br = BufReader::new($inner);
            while br.read_line(&mut buffer)? != 0 {
                $output.write_all(buffer.as_bytes())?;
                buffer.clear();
            }
        };
    };
    if let Some(child_stderr) = child.stderr {
        write!(child_stderr, stderr());
    }
    if let Some(child_stdout) = child.stdout {
        write!(child_stdout, stdout());
    }
    Ok(())
}

fn manage_flags<'a, 'b, 'c, 'd>(
    flags: &'a [&str],
    to_add_string: &'b str,
    to_subtract_string: &'c str,
) -> impl Iterator<Item = &'d str>
where
    'a: 'd,
    'b: 'd,
    'c: 'd,
{
    flags
        .iter()
        .filter_map(move |flag| {
            if to_subtract_string
                .split(',')
                .any(|to_subtract| to_subtract == *flag)
            {
                None
            } else {
                Some(*flag)
            }
        })
        .chain(to_add_string.split(','))
        .filter(|flag| !flag.is_empty())
}

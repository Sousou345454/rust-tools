use crate::manage_flags;
use std::io::{stdout, Write};

const RUST_FLAGS: [&str; 11] = [
    "-Dbad_style",
    "-Dfuture_incompatible",
    "-Dmissing_debug_implementations",
    "-Dmissing_docs",
    "-Dnonstandard_style",
    "-Drust_2018_compatibility",
    "-Drust_2018_idioms",
    "-Dtrivial_casts",
    "-Dunused_lifetimes",
    "-Dunused_qualifications",
    "-Dwarnings",
];

pub fn rust_flags(to_add_string: String, to_subtract_string: String) -> crate::Result<()> {
    let mut iter = manage_flags(&RUST_FLAGS, &to_add_string, &to_subtract_string);
    let mut s = String::with_capacity(iter.size_hint().0);
    if let Some(first) = iter.next() {
        s.push_str(first);
    }
    for element in iter {
        s.push(' ');
        s.push_str(element);
    }
    stdout().write_all(s.as_bytes())?;
    Ok(())
}

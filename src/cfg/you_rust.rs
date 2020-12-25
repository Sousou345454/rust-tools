use crate::Params;

const CLIPPY_FLAGS: &[&str] = &[
  "-Dclippy::restriction",
  "-Dwarnings",
  "-Aclippy::implicit_return",
  "-Aclippy::missing_docs_in_private_items",
];

const RUST_FLAGS: &[&str] = &[
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

const RUSTFMT_FLAGS: &[&str] = &[
  r#"edition="2018""#,
  "tab_spaces=2",
  "use_field_init_shorthand=true",
  r#"use_small_heuristics="Max""#,
];

#[derive(Debug, PartialEq)]
pub struct YouRust(pub Params);

impl Default for YouRust {
  fn default() -> Self {
    Self(Params {
      clippy_flags: CLIPPY_FLAGS.iter().map(|&e| e.into()).collect(),
      rust_flags: RUST_FLAGS.iter().map(|&e| e.into()).collect(),
      rustfmt_flags: RUSTFMT_FLAGS.iter().map(|&e| e.into()).collect(),
      toolchain: "".into(),
    })
  }
}

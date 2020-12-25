#[derive(Default)]
pub struct TransformingParams {
  pub add_clippy_flags: Vec<String>,
  pub add_rust_flags: Vec<String>,
  pub add_rustfmt_flags: Vec<String>,
  pub rm_clippy_flags: Vec<String>,
  pub rm_rust_flags: Vec<String>,
  pub rm_rustfmt_flags: Vec<String>,
  pub toolchain: String,
}

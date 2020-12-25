mod cargo;
mod rust_flags;
mod set_up;
mod tools;

use crate::Params;

pub struct Actions {
  pub params: Params,
}

impl Actions {
  #[inline]
  pub fn new(params: Params) -> Self {
    Self { params }
  }
}

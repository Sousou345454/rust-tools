use core::str::FromStr;

/// Action
pub enum Action {
    CheckGeneric,
    CheckWithFeatures,
    Clippy,
    RustFlags,
    Rustfmt,
    TestGeneric,
    TestWithFeatures,
}

impl FromStr for Action {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "check-with-features" => Self::CheckWithFeatures,
            "check-generic" => Self::CheckGeneric,
            "clippy" => Self::Clippy,
            "rust-flags" => Self::RustFlags,
            "rustfmt" => Self::Rustfmt,
            "test-generic" => Self::TestGeneric,
            "test-with-features" => Self::TestWithFeatures,
            _ => return Err(crate::Error::UnknownAction),
        })
    }
}

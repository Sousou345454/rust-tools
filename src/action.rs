use core::str::FromStr;

macro_rules! create_action {
    (
        $(#[$mac:meta])*
        $v:vis enum $enum_ident:ident {
            $first_variant_ident:ident, $first_variant_str:literal;
            $($variant_ident:ident, $variant_str:literal;)*
        }
    ) => {
        $(#[$mac])*
        $v enum $enum_ident {
          $first_variant_ident,
          $($variant_ident,)*
        }

        impl $enum_ident {
            #[inline]
            pub const fn list() -> &'static str {
                concat!(
                    $first_variant_str,
                    $(", ", $variant_str,)*
                )
            }
        }

        impl FromStr for Action {
            type Err = crate::Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $first_variant_str => Self::$first_variant_ident,
                    $($variant_str => Self::$variant_ident,)*
                    _ => return Err(crate::Error::UnknownAction),
                })
            }
        }
    }
}

create_action!(
    #[derive(Debug)]
    pub enum Action {
        BuildGeneric, "build-generic";
        BuildWithFeatures, "build-with-features";
        CheckGeneric, "check-generic";
        CheckWithFeatures, "check-with-features";
        Clippy, "clippy";
        RustFlags, "rust-flags";
        Rustfmt, "rustfmt";
        TestGeneric, "test-generic";
        TestWithFeatures, "test-with-features";
    }
);

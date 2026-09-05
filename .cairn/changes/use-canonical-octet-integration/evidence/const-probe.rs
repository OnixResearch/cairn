#![no_std]

extern crate alloc;

const ASCII_CONTROL_END_EXCLUSIVE: u8 = 32;
pub struct Failure;

macro_rules! define_candidates {
    ($($constant:tt)?) => {
        pub $($constant)? fn schema_matches(value: &alloc::string::String) -> bool {
            value == "schema"
        }

        pub $($constant)? fn has_prefix(value: &str) -> bool {
            value.starts_with("b3:")
        }

        pub $($constant)? fn control_free(value: &str) -> bool {
            !value.bytes().any(|byte| byte < ASCII_CONTROL_END_EXCLUSIVE)
        }

        impl core::fmt::Display for Failure {
            $($constant)? fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("failure")
            }
        }
    };
}

#[cfg(not(const_candidate))]
define_candidates!();

#[cfg(const_candidate)]
define_candidates!(const);

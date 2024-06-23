#![allow(clippy::redundant_static_lifetimes)]

pub mod environment_configuration {
    pub const ENVIRONMENT_CONFIGURATION_CONSTANT_NAME: &'static str = "ENVIRONMENT_CONFIGURATION";

    #[macro_export]
    macro_rules! environment_configuration_constant_file_name {
        () => {
            "environment_configuration_constant.rs"
        };
    }
}

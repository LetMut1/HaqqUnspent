use build_script_constant::environment_configuration_constant_file_name;

include!(
    concat!(
        env!("OUT_DIR"),
        concat!(
            "/",
            environment_configuration_constant_file_name!()
        )
    )
);

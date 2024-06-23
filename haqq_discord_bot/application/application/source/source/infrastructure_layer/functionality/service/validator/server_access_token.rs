use super::Validator;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;

pub use crate::infrastructure_layer::data::control_type::ServerAccessToken;

impl Validator<ServerAccessToken> {
    pub fn is_valid<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        server_access_token: &'a ServerAccessToken,
    ) -> bool {
        return server_access_token.0.as_str() == environment_configuration.security.server_access_token.as_str();
    }
}

use super::Validator;
pub use crate::infrastructure_layer::data::control_type::ServerAccessToken;
use crate::infrastructure_layer::data::environment_configuration::ENVIRONMENT_CONFIGURATION;

impl Validator<ServerAccessToken> {
    pub fn is_valid<'a>(server_access_token: &'a ServerAccessToken) -> bool {
        return server_access_token.0.as_str() == ENVIRONMENT_CONFIGURATION.security.server_access_token.0;
    }
}

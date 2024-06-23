use super::Resolver;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;

use serenity::all::ApplicationId;
pub use serenity::http::Http as SerenityHttp;

impl Resolver<SerenityHttp> {
    pub fn create<'a>(environment_configuration: &'a EnvironmentConfiguration) -> SerenityHttp {
        let serenity_http = SerenityHttp::new(environment_configuration.remote_service.discord.application.bot.token.as_str());

        serenity_http.set_application_id(ApplicationId::new(environment_configuration.remote_service.discord.application.bot.id));

        return serenity_http;
    }
}

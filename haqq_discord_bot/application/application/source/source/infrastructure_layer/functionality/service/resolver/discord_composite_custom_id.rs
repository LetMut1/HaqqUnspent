use super::Resolver;
use crate::application_layer::functionality::service::reactor::action_round___invalid_argument::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;

pub use crate::infrastructure_layer::data::control_type::DiscordCompositeCustomId;

impl Resolver<DiscordCompositeCustomId> {
    const DELIMITER: &'static str = ".";

    pub fn from_raffle_id_to_id<'a>(
        base_custom_id: &'a str,
        raffle_id: i64,
    ) -> String {
        return format!(
            "{}{}{}",
            base_custom_id,
            Self::DELIMITER,
            raffle_id
        );
    }

    pub fn from_id_to_raffle_id<'a>(
        base_custom_id: &'a str,
        discord_composit_custom_id: &'a str,
    ) -> Result<i64, Auditor<Error>> {
        let part_registry = discord_composit_custom_id.split(Self::DELIMITER).collect::<Vec<&'_ str>>();

        if part_registry.len() == 2 && part_registry[0] == base_custom_id {
            let raffle_id = match part_registry[1].parse::<i64>() {
                Ok(raffle_id_) => raffle_id_,
                Err(error) => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::Runtime {
                                runtime: Runtime::Other {
                                    other: Other::new(error),
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

            return Ok(raffle_id);
        }

        return Err(
            Auditor::<Error>::new(
                Error::create_invalid_value(),
                BacktracePart::new(
                    line!(),
                    file!(),
                ),
            ),
        );
    }
}

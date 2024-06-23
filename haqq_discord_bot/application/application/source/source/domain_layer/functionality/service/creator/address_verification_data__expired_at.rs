use super::Creator;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;

pub use crate::domain_layer::data::entity::address_verification_data::AddressVerificationData_ExpiredAt;

impl Creator<AddressVerificationData_ExpiredAt> {
    pub fn create<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        address_verification_data_created_at: i64,
    ) -> Result<i64, Auditor<Error>> {
        let address_verification_data_expired_at =
            match address_verification_data_created_at.checked_add(60 * environment_configuration.noncontext_parameters.wallet_verification_process_duraction_minutes) {
                Some(address_verification_data_expired_at_) => address_verification_data_expired_at_,
                None => {
                    return Err(
                        Auditor::<Error>::new(
                            Error::create_overflow_occured(),
                            BacktracePart::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
            };

        return Ok(address_verification_data_expired_at);
    }
}

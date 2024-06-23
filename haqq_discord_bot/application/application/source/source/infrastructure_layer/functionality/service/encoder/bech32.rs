use super::Encoder;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use bech32::Bech32;
use bech32::Hrp;

impl Encoder<Bech32> {
    const BECH32_HUMAN_READABLE_PREFIX: &'static str = "haqq";
    const EVM_ADDRESS_PREFIX: &'static str = "0x";

    pub fn encode<'a>(evm_address_with_prefix: &'a str) -> Result<String, Auditor<Error>> {
        let evm_address_without_prefix_ = match evm_address_with_prefix.strip_prefix(Self::EVM_ADDRESS_PREFIX) {
            Some(evm_address_without_prefix__) => evm_address_without_prefix__,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::Logic {
                            message: "Evm address should be with prefix.",
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        let hex_decoded_evm_address = match hex::decode(evm_address_without_prefix_.as_bytes()) {
            Ok(hex_decoded_evm_address_) => hex_decoded_evm_address_,
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

        let bech32_human_readable_prefix = match Hrp::parse(Self::BECH32_HUMAN_READABLE_PREFIX) {
            Ok(bech32_human_readable_prefix_) => bech32_human_readable_prefix_,
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

        let bech32_address = match bech32::encode::<Bech32>(
            bech32_human_readable_prefix,
            hex_decoded_evm_address.as_slice(),
        ) {
            Ok(bech32_address_) => bech32_address_,
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

        return Ok(bech32_address);
    }

    pub fn decode<'a>(bech32_address: &'a str) -> Result<String, Auditor<Error>> {
        let (human_readable_prefix, hex_decoded_evm_address) = match bech32::decode(bech32_address) {
            Ok(value) => value,
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

        if human_readable_prefix.as_str() != Self::BECH32_HUMAN_READABLE_PREFIX {
            return Err(
                Auditor::<Error>::new(
                    Error::Logic {
                        message: "Invalid bech32 human readable prefix."
                    },
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }

        let evm_address_without_prefix = hex::encode(hex_decoded_evm_address.as_slice());

        let evm_address_with_prefix = format!("{}{}", Self::EVM_ADDRESS_PREFIX, evm_address_without_prefix.as_str());

        return Ok(evm_address_with_prefix);
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::*;

    const REAL_HAQQ_NETWORK_MAINNET_EVM_ADDRESS_WITH_PREFIX: &'static str = "0x8f140f4bc40ea660b6a89e12c68dd4f1ca1d8bcc";
    const REAL_HAQQ_NETWORK_MAINNET_BECH32_ADDRESS: &'static str = "haqq13u2q7j7yp6nxpd4gncfvdrw5789pmz7v075sjv";

    // This shows that the algorithm matches the algorithm implemented on the Haqq-network
    #[test]
    fn encode() -> Result<(), Box<dyn Error + 'static>> {
        assert_eq!(
            Encoder::<Bech32>::encode(REAL_HAQQ_NETWORK_MAINNET_EVM_ADDRESS_WITH_PREFIX)?,
            REAL_HAQQ_NETWORK_MAINNET_BECH32_ADDRESS,
        );

        return Ok(());
    }

    #[test]
    fn decode() -> Result<(), Box<dyn Error + 'static>> {
        assert_eq!(
            REAL_HAQQ_NETWORK_MAINNET_EVM_ADDRESS_WITH_PREFIX,
            Encoder::<Bech32>::decode(Encoder::<Bech32>::encode(REAL_HAQQ_NETWORK_MAINNET_EVM_ADDRESS_WITH_PREFIX)?.as_str())?,
        );

        return Ok(());
    }
}
use super::Resolver;
use crate::application_layer::functionality::service::reactor::action_round___invalid_argument::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Other;
use crate::infrastructure_layer::data::error::Runtime;
use image::ImageFormat;
use image::Luma;
use qrcode::QrCode;
use std::io::Cursor;

pub use crate::infrastructure_layer::data::control_type::UTCDateTime;

impl Resolver<QrCode> {
    pub fn generate_byte_registry<D>(data: D) -> Result<Vec<u8>, Auditor<Error>>
    where
        D: AsRef<[u8]>,
    {
        let qr_code = match QrCode::new(data) {
            Ok(qr_code_) => qr_code_,
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

        let image_buffer = qr_code.render::<Luma<u8>>().build();

        let mut cursor = Cursor::new(Vec::<u8>::new());

        if let Err(error) = image_buffer.write_to(
            &mut cursor,
            ImageFormat::Png,
        ) {
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

        return Ok(cursor.into_inner());
    }
}

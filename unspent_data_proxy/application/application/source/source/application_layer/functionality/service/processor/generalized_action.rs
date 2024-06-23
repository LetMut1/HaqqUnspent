use super::Processor;
pub use crate::infrastructure_layer::data::control_type::GeneralizedAction;
use crate::{
    application_layer::{
        data::unified_report::UnifiedReport,
        functionality::service::reactor::Reactor,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            backtrace::BacktracePart,
            control_type::{
                ActionRound,
                Response,
            },
            error::Error,
            invalid_argument::InvalidArgument,
            invalid_argument_result::InvalidArgumentResult,
        },
        functionality::service::{
            creator::Creator,
            serializer::{
                Serialize,
                Serializer,
            },
            validator::Validator,
        },
    },
};
use clickhouse::Client;
use http::request::Parts;
use hyper::Body;
use matchit::Params;
use serde::Serialize as SerdeSerialize;
use std::future::Future;

impl Processor<GeneralizedAction> {
    pub async fn process<'a, 'b, 'c, DE, F1, AP, F2, I, O, P, SF>(
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        clickhouse_client: Client,
        data_extractor: DE,
        action_processor: AP,
    ) -> Response
    where
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<InvalidArgumentResult<Option<I>>, Auditor<Error>>>,
        AP: FnOnce(Option<I>, Client) -> F2,
        F2: Future<Output = Result<InvalidArgumentResult<UnifiedReport<O, P>>, Auditor<Error>>>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SF>: Serialize,
    {
        if !Validator::<Parts>::is_valid(parts) {
            let response = Creator::<Response>::create_bad_request();

            Reactor::<(
                ActionRound,
                Auditor<InvalidArgument>,
            )>::react(
                parts,
                &response,
                Auditor::<InvalidArgument>::new(
                    InvalidArgument::new(),
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                ),
            );

            return response;
        }

        let incoming = match data_extractor(
            body,
            parts,
            route_parameters,
        )
        .await
        {
            Ok(incoming_) => incoming_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                let response = Creator::<Response>::create_internal_server_error();

                Reactor::<(
                    ActionRound,
                    Auditor<Error>,
                )>::react(
                    parts,
                    &response,
                    error_auditor,
                );

                return response;
            }
        };

        let incoming_ = match incoming {
            InvalidArgumentResult::Ok {
                subject: incoming__,
            } => incoming__,
            InvalidArgumentResult::InvalidArgumentAuditor {
                mut invalid_argument_auditor,
            } => {
                invalid_argument_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                let response = Creator::<Response>::create_bad_request();

                Reactor::<(
                    ActionRound,
                    Auditor<InvalidArgument>,
                )>::react(
                    parts,
                    &response,
                    invalid_argument_auditor,
                );

                return response;
            }
        };

        let unified_report = match action_processor(
            incoming_,
            clickhouse_client,
        )
        .await
        {
            Ok(unified_report_) => unified_report_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                let response = Creator::<Response>::create_internal_server_error();

                Reactor::<(
                    ActionRound,
                    Auditor<Error>,
                )>::react(
                    parts,
                    &response,
                    error_auditor,
                );

                return response;
            }
        };

        let unified_report_ = match unified_report {
            InvalidArgumentResult::Ok {
                subject: unified_report__,
            } => unified_report__,
            InvalidArgumentResult::InvalidArgumentAuditor {
                mut invalid_argument_auditor,
            } => {
                invalid_argument_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                let response = Creator::<Response>::create_bad_request();

                Reactor::<(
                    ActionRound,
                    Auditor<InvalidArgument>,
                )>::react(
                    parts,
                    &response,
                    invalid_argument_auditor,
                );

                return response;
            }
        };

        let data = match Serializer::<SF>::serialize(&unified_report_) {
            Ok(data_) => data_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                let response = Creator::<Response>::create_internal_server_error();

                Reactor::<(
                    ActionRound,
                    Auditor<Error>,
                )>::react(
                    parts,
                    &response,
                    error_auditor,
                );

                return response;
            }
        };

        let response = Creator::<Response>::create_ok(Some(data));

        Reactor::<(
            ActionRound,
            Response,
        )>::react(
            parts,
            &response,
        );

        return response;
    }
}

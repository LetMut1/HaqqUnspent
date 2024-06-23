use super::Processor;
use crate::application_layer::data::unified_report::UnifiedReport;
use crate::application_layer::functionality::service::reactor::Reactor;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::backtrace::BacktracePart;
use crate::infrastructure_layer::data::control_type::ActionRound;
use crate::infrastructure_layer::data::control_type::Response;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::invalid_argument::InvalidArgument;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgumentResult;
use crate::infrastructure_layer::functionality::service::creator::Creator;
use crate::infrastructure_layer::functionality::service::serializer::Serialize;
use crate::infrastructure_layer::functionality::service::serializer::Serializer;
use crate::infrastructure_layer::functionality::service::validator::Validator;
use http::request::Parts;
use hyper::Body;
use matchit::Params;
use serde::Serialize as SerdeSerialize;
use std::future::Future;
use std::sync::Arc;

pub use crate::infrastructure_layer::data::control_type::GeneralizedAction;

impl Processor<GeneralizedAction> {
    pub async fn process_unified_report<'a, 'b, 'c, DE, F1, AP, F2, I, O, P, SF>(
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        environment_configuration: Arc<EnvironmentConfiguration>,
        data_extractor: DE,
        action_processor: AP,
    ) -> Response
    where
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<InvalidArgumentResult<Option<I>>, Auditor<Error>>>,
        AP: FnOnce(Option<I>, Arc<EnvironmentConfiguration>) -> F2,
        F2: Future<Output = Result<InvalidArgumentResult<UnifiedReport<O, P>>, Auditor<Error>>>,
        O: SerdeSerialize,
        P: SerdeSerialize,
        Serializer<SF>: Serialize,
    {
        let response_creator = move |unified_report: UnifiedReport<O, P>| -> Result<Response, Auditor<Error>> {
            let data = match Serializer::<SF>::serialize(&unified_report) {
                Ok(data_) => data_,
                Err(mut error_auditor) => {
                    error_auditor.add_backtrace_part(
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    );

                    return Err(error_auditor);
                }
            };

            return Ok(Creator::<Response>::create_ok(data));
        };

        return Self::process(
            body,
            parts,
            route_parameters,
            environment_configuration,
            data_extractor,
            action_processor,
            response_creator,
        )
        .await;
    }

    pub async fn process_option<'a, 'b, 'c, DE, F1, AP, F2, I, O, SF>(
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        environment_configuration: Arc<EnvironmentConfiguration>,
        data_extractor: DE,
        action_processor: AP,
    ) -> Response
    where
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<InvalidArgumentResult<Option<I>>, Auditor<Error>>>,
        AP: FnOnce(Option<I>, Arc<EnvironmentConfiguration>) -> F2,
        F2: Future<Output = Result<InvalidArgumentResult<Option<O>>, Auditor<Error>>>,
        O: SerdeSerialize,
        Serializer<SF>: Serialize,
    {
        let response_creator = move |outcoming: Option<O>| -> Result<Response, Auditor<Error>> {
            let response = match outcoming {
                Some(outcoming_) => {
                    let data = match Serializer::<SF>::serialize(&outcoming_) {
                        Ok(data_) => data_,
                        Err(mut error_auditor) => {
                            error_auditor.add_backtrace_part(
                                BacktracePart::new(
                                    line!(),
                                    file!(),
                                ),
                            );

                            return Err(error_auditor);
                        }
                    };

                    Creator::<Response>::create_ok(data)
                }
                None => Creator::<Response>::create_ok_(),
            };

            return Ok(response);
        };

        return Self::process(
            body,
            parts,
            route_parameters,
            environment_configuration,
            data_extractor,
            action_processor,
            response_creator,
        )
        .await;
    }

    async fn process<'a, 'b, 'c, DE, F1, AP, F2, RC, I, O, SF>(
        body: &'a mut Body,
        parts: &'a Parts,
        route_parameters: &'a Params<'b, 'c>,
        environment_configuration: Arc<EnvironmentConfiguration>,
        data_extractor: DE,
        action_processor: AP,
        response_creator: RC,
    ) -> Response
    where
        DE: FnOnce(&'a mut Body, &'a Parts, &'a Params<'b, 'c>) -> F1,
        F1: Future<Output = Result<InvalidArgumentResult<Option<I>>, Auditor<Error>>>,
        AP: FnOnce(Option<I>, Arc<EnvironmentConfiguration>) -> F2,
        F2: Future<Output = Result<InvalidArgumentResult<O>, Auditor<Error>>>,
        RC: FnOnce(O) -> Result<Response, Auditor<Error>>,
        O: SerdeSerialize,
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

        let outcoming = match action_processor(
            incoming_,
            environment_configuration,
        )
        .await
        {
            Ok(outcoming_) => outcoming_,
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

        let outcoming_ = match outcoming {
            InvalidArgumentResult::Ok {
                subject: outcoming__,
            } => outcoming__,
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

        let response = match response_creator(outcoming_) {
            Ok(response_) => response_,
            Err(mut error_auditor) => {
                error_auditor.add_backtrace_part(
                    BacktracePart::new(
                        line!(),
                        file!(),
                    ),
                );

                let response_ = Creator::<Response>::create_internal_server_error();

                Reactor::<(
                    ActionRound,
                    Auditor<Error>,
                )>::react(
                    parts,
                    &response_,
                    error_auditor,
                );

                return response_;
            }
        };

        Reactor::<(
            ActionRound,
            Response,
        )>::react(
            parts, &response,
        );

        return response;
    }
}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_match,
    clippy::explicit_into_iter_loop,
    clippy::module_inception,
    clippy::needless_continue,
    clippy::needless_lifetimes,
    clippy::needless_return,
    clippy::new_without_default,
    clippy::redundant_pattern_matching,
    clippy::redundant_static_lifetimes,
    clippy::single_match_else,
    clippy::string_add,
    clippy::too_many_arguments,
    clippy::trait_duplication_in_bounds,
    clippy::unused_unit,
    clippy::empty_enum,
    clippy::let_unit_value
)]
#![deny(
    clippy::unnecessary_cast,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::fallible_impl_from,
    clippy::float_cmp_const,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::mem_forget,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values
)]

use application::{
    application_layer::functionality::command_processor::{
        create_fixtures::CreateFixtures,
        run_server::RunServer,
        CommandProcessor,
    },
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            error::Error,
        },
        functionality::service::formatter::Formatter,
    },
};
use clap::{
    command,
    Command,
};
use std::error::Error as StdError;

const RUN_SERVER: &'static str = "run_server";
const CREATE_FIXTURES: &'static str = "create_fixtures";

fn main() -> Result<(), Box<dyn StdError + 'static>> {
    if let Err(error) = process() {
        println!(
            "{}",
            &error
        );

        return Err(error);
    }

    return Ok(());
}

fn process() -> Result<(), Box<dyn StdError + 'static>> {
    let arg_matches = command!()
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new(RUN_SERVER))
        .subcommand(Command::new(CREATE_FIXTURES))
        .get_matches();

    let subcommand_arg_matches = match arg_matches.subcommand() {
        Some(subcommand_arg_matches_) => subcommand_arg_matches_,
        None => {
            return Err("Logic error. Exhausted list of subcommands.".into());
        }
    };

    match subcommand_arg_matches {
        (RUN_SERVER, _) => {
            if let Err(error_auditor) = CommandProcessor::<RunServer>::process() {
                return Err(Formatter::<Auditor<Error>>::format(&error_auditor).into());
            }
        }
        (CREATE_FIXTURES, _) => {
            if let Err(error_auditor) = CommandProcessor::<CreateFixtures>::process() {
                return Err(Formatter::<Auditor<Error>>::format(&error_auditor).into());
            }
        }
        _ => {
            return Err("Unexpexted subcommand.".into());
        }
    }

    return Ok(());
}

// TODO: Если в текущей архитектуре Снепшотов не будет хватать скорости, то либо сделать одну денормализованную таблицу, либо перейти к Aggregation или Join движок со скриптами обновления таблиц.
// TODO: В запросах из Макросов  LIMIT 1 BY заменить GroupBy + WindowFunction;
// TODO: Возможно, через подзапросы запросы из Макросов будут выполняться быстрее.
// TODO: УБрать все временные команды.
// TODO: После вынесения сервиса на первую линию проверить HTTP-методы. Некоторые из них были изменены из-за ограничений инфраструктуры, но она уже поменялась.
// TODO: Почистить данные от нулевых (= 0) значений балансев, если есть.
// TODO: Проверить в Постгресе и Кликхаусе отрицательные значения балансев.
// TODO: Для запросов на сервис авторизации юзером можно сделать ssl-pinning

// TODO: убрать newtype паттерн, но оставить контекстную архитектуру черех фантомную дату.

// acceskey accestoken -> server acces token    user access token

// Переделать фикстуры. Нужны ли для Постгреса.

//  let naive_date_time = match NaiveDateTime::from_timestamp_opt(    - через сервис постаратьс сдлать

// лру кеш

// InvalidArgument через трейсинг и контекст.

// Перенести DTO в queried

// коннекшн к кликхаусу часто закрывается. Смотреть на http://127.0.0.1:80/asset_snapshot/history дев окружении.

// TODO описать В потсгресе для Ассета:
// Перенести расчет в ассеты процентов - сейчас это заюирается с постгреса.

// При изменениии любого параметра в бд Постгрес (например, Лэйбл в кошельке), нужно сделать запро на замену в кликзаусе, либо подождать около 15 минут.

// [remote_service.user_authorization.url] сделать как в генераторе.
// [remote_service.unspen.url] - адрес не точки, а просто домен

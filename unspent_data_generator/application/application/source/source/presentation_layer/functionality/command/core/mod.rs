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
        run_all_tasks::RunAllTasks,
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
use std::{
    env::var,
    error::Error as StdError,
};

const RUN_ALL_TASKS: &'static str = "run_all_tasks";

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
    let arg_matches = command!().arg_required_else_help(true).subcommand_required(true).subcommand(Command::new(RUN_ALL_TASKS)).get_matches();

    let subcommand_arg_matches = match arg_matches.subcommand() {
        Some(subcommand_arg_matches_) => subcommand_arg_matches_,
        None => {
            return Err("Logic error. Exhausted list of subcommands.".into());
        }
    };

    match subcommand_arg_matches {
        (RUN_ALL_TASKS, _) => {
            let environment_configuration_directory_path_part = var("CARGO_MANIFEST_DIR")?;

            let environment_configuration_directory_path = format!(
                "{}/../../../../../../environment_configuration",
                environment_configuration_directory_path_part.as_str(),
            );

            if let Err(error_auditor) = CommandProcessor::<RunAllTasks>::process(environment_configuration_directory_path.as_str()) {
                return Err(Formatter::<Auditor<Error>>::format(&error_auditor).into());
            }
        }
        _ => {
            return Err("Unexpexted subcommand.".into());
        }
    }

    return Ok(());
}

// TODO: перейти на новую версию языка.
// TODO: уйти от НьюТайпПаттерн в домене.

// TODO  asset deleting, getinfo, desctop tasks

// дата-прокси ServerAccessToken,  UserAccesToken

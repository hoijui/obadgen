// SPDX-FileCopyrightText: 2021 - 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#![feature(trait_alias)]
#![warn(rust_2021_compatibility)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![warn(clippy::wildcard_enum_match_arm)]
#![warn(clippy::string_slice)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::try_err)]
#![warn(clippy::shadow_reuse)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::else_if_without_else)]
#![warn(clippy::use_debug)]
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::indexing_slicing)]
// NOTE allowed because:
//      If the same regex is going to be applied to multiple inputs,
//      the precomputations done by Regex construction
//      can give significantly better performance
//      than any of the `str`-based methods.
#![allow(clippy::trivial_regex)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::fn_params_excessive_bools)]

use box_err::BoxResult;
use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command, ValueHint};
use const_format::formatcp;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::path::PathBuf;

pub mod box_err;
mod constants;
mod environment;
mod hash;
mod logger;
pub mod open_badge;
mod patcher;
mod process;
pub mod settings;
mod signature;
mod std_error;
#[cfg(test)]
mod test_util;

use crate::environment::Environment;
use crate::settings::{Settings, Verbosity};

pub const A_L_VERSION: &str = "version";
pub const A_S_VERSION: char = 'V';
const A_S_PROJECT_ROOT: char = 'C';
const A_L_PROJECT_ROOT: &str = "project-root";
const A_L_RAW_PANIC: &str = "raw-panic";
const A_S_FILE_OUT: char = 'O';
const A_L_FILE_OUT: &str = "file-out";
const A_S_VERBOSE: char = 'v';
const A_L_VERBOSE: &str = "verbose";
const A_S_LOG_LEVEL: char = 'F';
const A_L_LOG_LEVEL: &str = "log-level";
const A_S_QUIET: char = 'q';
const A_L_QUIET: &str = "quiet";
const A_S_OVERWRITE: char = 'o';
const A_L_OVERWRITE: &str = "overwrite";
const A_S_LIST: char = 'l';
const A_L_LIST: &str = "list";
const A_S_DATE_FORMAT: char = 'T';
const A_L_DATE_FORMAT: &str = "date-format";

fn arg_version() -> Arg {
    Arg::new(A_L_VERSION)
        .help(formatcp!("Print version information and exit. May be combined with -{A_S_QUIET},--{A_L_QUIET}, to really only output the version string."))
        .short(A_S_VERSION)
        .long(A_L_VERSION)
        .action(ArgAction::SetTrue)
}

fn arg_project_root() -> Arg {
    Arg::new(A_L_PROJECT_ROOT)
        .help("The root dir of the project")
        .long_help(
            "The root directory of the project, \
            mainly used for SCM (e.g. git) information gathering.",
        )
        .num_args(1)
        .value_parser(value_parser!(std::path::PathBuf))
        .value_name("DIR")
        .value_hint(ValueHint::DirPath)
        .short(A_S_PROJECT_ROOT)
        .long(A_L_PROJECT_ROOT)
        .action(ArgAction::Set)
        .required(false)
        .default_value(".")
}

fn arg_raw_panic() -> Arg {
    Arg::new(A_L_RAW_PANIC)
        .help("Use rusts native panic handling, if one occures.")
        .long_help(
            "Do not wrap rusts native panic handling functionality \
            in a more end-user-friendly way. \
            This is particularly useful when running on CI.",
        )
        .action(ArgAction::SetTrue)
        .long(A_L_RAW_PANIC)
}

fn arg_out_file() -> Arg {
    Arg::new(A_L_FILE_OUT)
        .help("Write variables into this file; .env or .json")
        .long_help(
            "Write evaluated values into a file. \
            Two file formats are supported: \
            * ENV: one KEY=VALUE pair per line (BASH syntax) \
            * JSON: a dictionary of KEY: \"value\" \
            You can choose which format is used by the file-extension.
            Note that \"-\" has no special meaning here; \
            it does not mean stdout, but rather the file \"./-\".",
        )
        .num_args(1)
        .value_parser(value_parser!(std::path::PathBuf))
        .value_name("FILE")
        .value_hint(ValueHint::FilePath)
        .short(A_S_FILE_OUT)
        .long(A_L_FILE_OUT)
        .action(ArgAction::Set)
        // .default_value(sinks::DEFAULT_FILE_OUT)
        .required(false)
}

fn arg_verbose() -> Arg {
    Arg::new(A_L_VERBOSE)
        .help("More verbose log output")
        .long_help(formatcp!(
            "More verbose log output; useful for debugging. \
            See -{A_S_LOG_LEVEL},--{A_L_LOG_LEVEL} for more fine-graine control.",
        ))
        .short(A_S_VERBOSE)
        .long(A_L_VERBOSE)
        .action(ArgAction::Count)
        .required(false)
}

fn arg_log_level() -> Arg {
    Arg::new(A_L_LOG_LEVEL)
        .help("Set the log-level")
        .value_parser(value_parser!(settings::Verbosity))
        .short(A_S_LOG_LEVEL)
        .long(A_L_LOG_LEVEL)
        .action(ArgAction::Set)
        .required(false)
        .conflicts_with(A_L_VERBOSE)
        .conflicts_with(A_L_QUIET)
}

fn arg_quiet() -> Arg {
    Arg::new(A_L_QUIET)
        .help("Minimize or suppress output to stdout")
        .long_help(formatcp!(
            "Minimize or suppress output to stdout, \
and only shows log output on stderr. \
See -{A_S_LOG_LEVEL},--{A_L_LOG_LEVEL} to also disable the later. \
This does not affect the log level for the log-file.",
        ))
        .action(ArgAction::SetTrue)
        .short(A_S_QUIET)
        .long(A_L_QUIET)
        .required(false)
        .conflicts_with(A_L_VERBOSE)
}

fn arg_overwrite() -> Arg {
    Arg::new(A_L_OVERWRITE)
        .help("Whether to overwrite already set values in the output.")
        .num_args(1)
        .value_parser(value_parser!(settings::Overwrite))
        .short(A_S_OVERWRITE)
        .long(A_L_OVERWRITE)
        .action(ArgAction::Set)
        .required(false)
    // .conflicts_with(A_L_DRY)
}

fn arg_list() -> Arg {
    Arg::new(A_L_LIST)
        .help("Show all properties and their keys")
        .long_help(
            "Prints a list of all the environment variables \
            that are potentially set by this tool onto stdout and exits.",
        )
        .action(ArgAction::SetTrue)
        .short(A_S_LIST)
        .long(A_L_LIST)
        .required(false)
}

fn arg_date_format() -> Arg {
    Arg::new(A_L_DATE_FORMAT)
        .help("Date format for generated dates")
        .long_help(
            "Date format string for generated (vs supplied) dates. \
            For details, see https://docs.rs/chrono/latest/chrono/format/strftime/index.html",
        )
        .num_args(1)
        .value_parser(clap::builder::NonEmptyStringValueParser::new()) // TODO Maybe parse directly into a date format?
        .value_hint(ValueHint::Other)
        .short(A_S_DATE_FORMAT)
        .long(A_L_DATE_FORMAT)
        .action(ArgAction::Set)
        .default_value(constants::DATE_FORMAT_GIT)
        .required(false)
}

lazy_static! {
    static ref ARGS: [Arg; 10] = [
        arg_version(),
        arg_project_root(),
        arg_raw_panic(),
        arg_out_file(),
        arg_verbose(),
        arg_log_level(),
        arg_quiet(),
        arg_overwrite(),
        arg_list(),
        arg_date_format(),
    ];
}

fn find_duplicate_short_options() -> Vec<char> {
    let mut short_options: Vec<char> = ARGS.iter().filter_map(clap::Arg::get_short).collect();
    // standard option --help
    short_options.push('h');
    // standard option --version
    // short_options.push('V'); // NOTE We handle this manually now
    short_options.sort_unstable();
    let mut duplicate_short_options = HashSet::new();
    let mut last_chr = '&';
    for chr in &short_options {
        if *chr == last_chr {
            duplicate_short_options.insert(*chr);
        }
        last_chr = *chr;
    }
    duplicate_short_options.iter().copied().collect()
}

fn arg_matcher() -> Command {
    let app = command!()
        .bin_name(clap::crate_name!())
        .help_expected(true)
        .disable_version_flag(true)
        .args(ARGS.iter());
    let duplicate_short_options = find_duplicate_short_options();
    assert!(
        duplicate_short_options.is_empty(),
        "Duplicate argument short options: {duplicate_short_options:?}"
    );
    app
}

// fn hosting_type(args: &ArgMatches) -> HostingType {
//     let hosting_type = args
//         .get_one::<HostingType>(A_L_HOSTING_TYPE)
//         .copied()
//         .unwrap_or_default();

//     if log::log_enabled!(log::Level::Debug) {
//         let hosting_type_str: &str = hosting_type.into();
//         log::debug!("Hosting-type setting: {}", hosting_type_str);
//     }

//     hosting_type
// }

fn overwrite(args: &ArgMatches) -> settings::Overwrite {
    let overwrite = args
        .get_one::<settings::Overwrite>(A_L_OVERWRITE)
        .copied()
        .unwrap_or_default();

    if log::log_enabled!(log::Level::Debug) {
        let overwrite_str: &str = overwrite.into();
        log::debug!("Overwriting output variable values? -> {}", overwrite_str);
    }

    overwrite
}

/// Returns the logging verbositiy to be used.
/// We only log to stderr;
/// if the user wnats to log anywere else,
/// they have to redirect from there.
/// We are simple enough to not having to worry about
/// complex logging schemes.
/// ... right? :/
fn verbosity(args: &ArgMatches) -> Verbosity {
    if args.get_flag(A_L_QUIET) {
        Verbosity::None
    } else if let Some(specified) = args.get_one::<Verbosity>(A_L_LOG_LEVEL).copied() {
        specified
    } else {
        // Set the default base level
        let level = if cfg!(debug_assertions) {
            Verbosity::Debug
        } else {
            Verbosity::Info
        };
        let num_verbose = *args.get_one::<u8>(A_L_VERBOSE).unwrap_or(&0);
        level.up_max(num_verbose)
    }
}

fn repo_path(args: &ArgMatches) -> PathBuf {
    let repo_path = args
        .get_one::<PathBuf>(A_L_PROJECT_ROOT)
        .cloned()
        .unwrap_or_else(PathBuf::new);
    log::debug!("Using repo path '{:#?}'.", &repo_path);
    repo_path
}

fn date_format(args: &ArgMatches) -> &str {
    let date_format = match args.get_one::<String>(A_L_DATE_FORMAT) {
        Some(date_format) => date_format,
        None => constants::DATE_FORMAT_GIT,
    };
    log::debug!("Using date format '{}'.", date_format);
    date_format
}

fn print_version_and_exit(quiet: bool) {
    #![allow(clippy::print_stdout)]

    if !quiet {
        print!("{} ", clap::crate_name!());
    }
    println!("{}", obadgen::VERSION);
    std::process::exit(0);
}

fn main() -> BoxResult<()> {
    let log_filter_reload_handle = logger::setup_logging()?;

    let args = arg_matcher().get_matches();

    // if !args.get_flag(A_L_RAW_PANIC) {
    //     human_panic::setup_panic!();
    // }

    let quiet = args.get_flag(A_L_QUIET);

    let version = args.get_flag(A_L_VERSION);
    if version {
        print_version_and_exit(quiet);
    }

    let verbosity = verbosity(&args);
    logger::set_log_level(&log_filter_reload_handle, verbosity)?;

    if args.get_flag(A_L_LIST) {
        let environment = Environment::stub();
        // let list = var::list_keys(&environment);
        // log::info!("{}", list);
        return Ok(());
    }

    let repo_path = repo_path(&args);
    let date_format = date_format(&args);

    let overwrite = overwrite(&args);

    let settings = Settings {
        repo_path: Some(repo_path),
        date_format: date_format.to_owned(),
        overwrite,
        verbosity,
    };
    log::trace!("Created Settings.");
    let mut environment = Environment::new(settings);
    log::trace!("Created Environment.");

    process::run(&mut environment)
}

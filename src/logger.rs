// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::io;

use obadgen::box_err::BoxResult;
use obadgen::settings::Verbosity;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    fmt,
    prelude::*,
    reload::{self, Handle},
    Registry,
};

const fn verbosity_to_level(verbosity: Verbosity) -> LevelFilter {
    match verbosity {
        Verbosity::None => LevelFilter::OFF,
        Verbosity::Errors => LevelFilter::ERROR,
        Verbosity::Warnings => LevelFilter::WARN,
        Verbosity::Info => LevelFilter::INFO,
        Verbosity::Debug => LevelFilter::DEBUG,
        Verbosity::Trace => LevelFilter::TRACE,
    }
}

/// Sets up logging, with a way to change the log level later on,
/// and with all output going to stderr,
/// as suggested by <https://clig.dev/>.
///
/// # Errors
///
/// If initializing the registry (logger) failed.
pub fn setup_logging() -> BoxResult<Handle<LevelFilter, Registry>> {
    let level_filter = if cfg!(debug_assertions) {
        // LevelFilter::DEBUG
        LevelFilter::TRACE
    } else {
        LevelFilter::INFO
    };
    let (filter, reload_handle_filter) = reload::Layer::new(level_filter);

    let l_stderr = fmt::layer().map_writer(move |_| io::stderr);

    let registry = tracing_subscriber::registry().with(filter).with(l_stderr);
    registry.try_init()?;

    Ok(reload_handle_filter)
}

pub fn set_log_level(
    reload_handle: &Handle<LevelFilter, Registry>,
    verbosity: Verbosity,
) -> BoxResult<()> {
    let level_filter = verbosity_to_level(verbosity);
    reload_handle.modify(|filter| *filter = level_filter)?;
    Ok(())
}

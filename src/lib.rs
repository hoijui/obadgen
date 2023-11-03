// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod constants;
pub mod environment;
pub mod hash;
pub mod open_badge;
pub mod patcher;
pub mod process;
pub mod settings;
pub mod signature;
pub mod std_error;

use git_version::git_version;

/// This serves as a very general, catch-all error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type BoxResult<T> = Result<T, BoxError>;

pub const VERSION: &str = git_version!();

/// This is an [officially endorsed](
/// https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#include-items-only-when-collecting-doctests)
/// hack to include rust code embedded in README.md when doc-testing.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;

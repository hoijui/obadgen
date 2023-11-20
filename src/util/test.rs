// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

// As of November 2023, this (trait alias)
// would require `#![feature(trait_alias)]` in lib.rs,
// and woudl cause compilation in rust stable to fail.
// #[cfg(test)]
// pub trait Normal = Sized + Send + Sync + Unpin;

#[cfg(test)]
pub fn is_normal<T: Sized + Send + Sync + Unpin>() {}

/// This ensures that the supplied type is a comfortably convertible,
/// wrapable and so on error type.
/// This should be used to check on (at least)
/// all public errors of a codebase.
#[cfg(test)]
pub fn is_good_error<T: Sized + Send + Sync + Unpin + std::error::Error>() {}

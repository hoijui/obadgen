// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

#[cfg(test)]
pub trait Normal = Sized + Send + Sync + Unpin;

#[cfg(test)]
pub fn is_normal<T: Normal>() {}

/// This ensures that the supplied type is a comfortably convertible,
/// wrapable and so on error type.
/// This should be used to check on (at least)
/// all public errors of a codebase.
#[cfg(test)]
pub fn is_good_error<T: Normal + std::error::Error>() {}

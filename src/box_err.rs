// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// This serves as a very general, catch-all error type.
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
/// This serves as a very general, catch-all result type.
pub type BoxResult<T> = Result<T, BoxError>;

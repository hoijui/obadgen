// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod png;
pub mod svg;

use crate::box_err::BoxError;
use std::path::Path;

use thiserror::Error;

/// This serves to wrap/represent `std::**()` `Option` return values as `Result`s,
/// like the one of [`std::fs::PathBuf::file_name()`], or [`std::OsStr::to_str()`].
#[derive(Error, Debug)]
pub enum Error {
    #[error("'verify' is already set to a different value.")]
    VerifyAlreadySet { present: String, proposed: String },

    // /// A required properties value could not be evaluated
    // #[error("The file name ends in \"..\", and does therefore not represent a file.")]
    // PathNotAFile,

    // #[error(
    //     "The string is not valid UTF-8, and can thus not be represented by a normal rust string."
    // )]
    // NotValidUtf8,

    // #[error(transparent)]
    // InvalidUrl(#[from] url::ParseError),
    /// Represents all cases of `std::io::Error`.
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Represents all other cases of `std::error::Error`,
    /// and even those without that trait.
    /// This includes format (SVG, PNG) specific errors.
    #[error(transparent)]
    Boxed(#[from] BoxError),
    // /// Represents all other cases of `std::error::Error`,
    // /// and even those without that trait.
    // /// This includes format (SVG, PNG) specific errors.
    // #[error(transparent)]
    // ThisErr(#[from] xml::reader::Error),
}

pub trait Patcher {
    /// Rewrites ("bakes" in Open Badge terms) an image file,
    /// adding Open Badge meta-data.
    /// 
    /// # Errors
    /// 
    /// Reading the source file failed.
    /// 
    /// Manipulating the data failed.
    /// 
    /// Writing the target file failed.
    fn rewrite<P: AsRef<Path>, S: AsRef<str>>(
        input_file: P,
        output_file: P,
        verify: S,
        fail_if_verify_present: bool,
    ) -> Result<(), Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::is_good_error;

    #[test]
    fn normal_types() {
        is_good_error::<Error>();
    }
}

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod png;
pub mod svg;

use crate::box_err::BoxError;
use std::path::{Path, PathBuf};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ToImageTypeError {
    #[error("Failed to extract file extension from path: '{path}'")]
    ExtensionExtraction { msg: &'static str, path: PathBuf },

    #[error("The only supported image file types are AVG and PNG; supplied extension: '{ext}' (in path: '{path}')")]
    Unsupported { ext: String, path: PathBuf },
}

/// This serves to wrap/represent `std::**()` `Option` return values as `Result`s,
/// like the one of [`std::fs::PathBuf::file_name()`], or [`std::OsStr::to_str()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Svg,
    Png,
}

impl TryFrom<&Path> for ImageType {
    type Error = ToImageTypeError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let ext = value
            .extension()
            .ok_or_else(|| ToImageTypeError::ExtensionExtraction {
                msg: "No file extension present",
                path: value.to_path_buf(),
            })?
            .to_str()
            .ok_or_else(|| ToImageTypeError::ExtensionExtraction {
                msg: "File extension is not UTF-8 compatible",
                path: value.to_path_buf(),
            })?;

        let ext_lower = ext.to_lowercase();
        match ext_lower.as_ref() {
            "svg" => Ok(Self::Svg),
            "png" => Ok(Self::Png),
            _ => Err(ToImageTypeError::Unsupported {
                ext: ext.to_string(),
                path: value.to_path_buf(),
            }),
        }
    }
}

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

pub(crate) trait Patcher {
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
        // &self,
        input_file: P,
        output_file: P,
        verify: S,
        fail_if_verify_present: bool,
    ) -> Result<(), Error>;
}

// pub enum AllPatcher {
//     SvgPatcher(svg::Patcher),
//     PngPatcher(png::Patcher),
// }

// impl Patcher for AllPatcher {
//     fn rewrite<P: AsRef<Path>, S: AsRef<str>>(
//         &self,
//         input_file: P,
//         output_file: P,
//         verify: S,
//         fail_if_verify_present: bool,
//     ) -> Result<(), Error> {
//         self.0.rewrite(input_file, output_file, verify, fail_if_verify_present)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::is_good_error;

    #[test]
    fn normal_types() {
        is_good_error::<Error>();
    }
}

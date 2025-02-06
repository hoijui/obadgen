// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use clap::ValueEnum;
use lazy_static::lazy_static;
use std::path::PathBuf;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, IntoStaticStr, VariantNames};

use crate::signature::Algorithm;

#[derive(
    Debug,
    ValueEnum,
    EnumString,
    VariantNames,
    EnumIter,
    Eq,
    IntoStaticStr,
    PartialEq,
    PartialOrd,
    Copy,
    Clone,
)]
pub enum Verbosity {
    None,
    Errors,
    Warnings,
    Info,
    Debug,
    Trace,
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Info
    }
}

lazy_static! {
    static ref VARIANTS_LIST: Vec<Verbosity> = Verbosity::iter().collect();
}

impl Verbosity {
    const fn index(self) -> usize {
        self as usize
    }

    /// Increases the verbosity by `steps`,
    /// halting at the upper bound of the enum.
    #[must_use]
    pub fn up_max<S: Into<usize>>(self, steps: S) -> Self {
        let new_index = self.index().saturating_add(steps.into()) % VARIANTS_LIST.len();
        VARIANTS_LIST[new_index]
    }

    /// Decreases the verbosity by `steps`,
    /// halting at the lower bound of the enum.
    #[must_use]
    pub fn down_max(self, steps: usize) -> Self {
        let new_index = self.index().saturating_sub(steps);
        VARIANTS_LIST[new_index]
    }
}

impl From<bool> for Verbosity {
    fn from(verbose: bool) -> Self {
        if verbose {
            Self::Info
        } else {
            Self::Warnings
        }
    }
}

#[derive(Debug, ValueEnum, EnumString, VariantNames, IntoStaticStr, Clone, Copy)]
pub enum Overwrite {
    All,
    None,
    Main,
    Alternative,
}

impl Overwrite {
    #[must_use]
    pub const fn main(&self) -> bool {
        match self {
            Self::All | Self::Main => true,
            Self::None | Self::Alternative => false,
        }
    }

    #[must_use]
    pub const fn alt(&self) -> bool {
        match self {
            Self::All | Self::Alternative => true,
            Self::None | Self::Main => false,
        }
    }
}

impl Default for Overwrite {
    fn default() -> Self {
        Self::All
    }
}

/* impl strum::VariantNames for Overwrite { */
/*     const VARIANTS: &'static [&'static str]; */
/* } */

#[derive(Clone, Copy, Debug)]
pub enum FailOn {
    AnyMissingValue,
    Error,
}

impl From<bool> for FailOn {
    fn from(verbose: bool) -> Self {
        if verbose {
            Self::AnyMissingValue
        } else {
            Self::Error
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShowRetrieved {
    No,
    Primary(Option<PathBuf>),
    All(Option<PathBuf>),
}

#[derive(Clone, Debug, Default)]
pub struct Settings /*<S: ::std::hash::BuildHasher>*/ {
    // pub repo_path: Option<Box<Path>>,
    // pub repo_path: Option<PathBuf>,
    // pub required_keys: HashSet<Key>,
    // pub overwrite: Overwrite,
    // pub date_format: String,
    // pub fail_on: FailOn,
    // vars: Box<HashMap<String, String, S>>,
    // #[builder(default = false)]
    // fail_on_missing: bool,
    // pub show_retrieved: ShowRetrieved,
    // pub hosting_type: HostingType,
    // pub only_required: bool,
    // pub key_prefix: Option<String>,
    pub verbosity: Verbosity,
    /// Location of the Open Badge Assertion JSON-LD to be baked.
    pub assertion_loc: Option<PathBuf>,
    /// Location of the private key required for signing,
    /// if signing is used.
    pub sign_alg: Algorithm,
    /// Location of the private key required for signing,
    /// if signing is used.
    pub sign_key_loc: Option<PathBuf>,
    /// Location of the certification chain,
    /// optionally incorporated if signing is used.
    pub cert_loc: Option<PathBuf>,
    /// Location of the to be baked Open Badge image.
    pub source_image_loc: Option<PathBuf>,
    /// Location of the to be baked Open Badge image.
    pub baked_loc: Option<PathBuf>,
}

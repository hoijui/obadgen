// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod box_err;
pub mod cert_gen;
pub mod constants;
pub mod hash;
mod objects;
pub mod patcher;
pub mod process;
pub mod serde_date_time;
pub mod settings;
pub mod signature;
pub mod std_error;
pub mod util;

pub use objects::assertion::Obj as Assertion;
pub use objects::badge_class::Obj as BadgeClass;
pub use objects::cryptographic_key::Obj as CryptographicKey;
pub use objects::evidence::Obj as Evidence;
pub use objects::identity::Obj as Identity;
pub use objects::identity::ObjType as IdentityType;
pub use objects::issuer::Obj as Issuer;
pub use objects::verification::Obj as Verification;
pub use objects::verification::ObjType as VerificationType;
pub use objects::ToJsonLd;

use git_version::git_version;

pub const VERSION: &str = git_version!();

/// This is an [officially endorsed](
/// https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#include-items-only-when-collecting-doctests)
/// hack to include rust code embedded in README.md when doc-testing.
#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDocTests;

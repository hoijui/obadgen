// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod assertion;
pub mod badge_class;
pub mod criteria;
pub mod cryptographic_key;
pub mod evidence;
pub mod identity;
pub mod issuer;
pub mod verification;

use serde::Serialize;

pub trait ToJsonLd
where
    Self: Serialize,
{
    /// This should generate Open Badge 2.0 compatible JSON-LD
    /// that represents the specific type.
    /// Each implementing object is responsible itsself
    /// to ensure that its JSON serde serialization
    /// is valid JSON-LD according to the Open Badge 2.0 Specification.
    ///
    /// # Errors
    ///
    /// If serde serialization fails.
    fn to_json_ld(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

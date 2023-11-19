// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// The property by which the recipient of a badge is identified.
/// This value should be an IRI mapped in the present context.
/// For example, `email` maps to <http://schema.org/email>
/// and indicates that the identity of the `IdentityObject`
/// will represent a value of a `Profile’s` `email` property.
/// See [Profile Identifier Properties](
/// http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#ProfileIdentifierProperties).
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
// #[serde(tag = "type")]
pub enum ObjType {
    #[default]
    EMail,
    Url,
    Telephone,
    // TODO DID?
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "lowercase")]
// #[serde(rename = "IdentityObject")]
pub struct Obj {
    pub r#type: ObjType,
    /// Either the hash of the identity or the plaintext value.
    /// If it’s possible that the plaintext transmission and storage
    /// of the identity value would leak personally identifiable information
    /// where there is an expectation of privacy,
    /// it is strongly recommended that an `IdentityHash` be used.
    #[builder(setter(into))]
    pub identity: String,
    /// Whether or not the identity value is hashed.
    pub hashed: bool,
    /// If the recipient is hashed,
    /// this should contain the string used to salt the hash.
    /// If this value is not provided,
    /// it should be assumed that the hash was not salted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    #[builder(setter(into))]
    pub salt: Option<String>,
}

impl Obj {
    pub fn new<S: Into<String>>(r#type: ObjType, identity: S, hashed: bool) -> Self {
        Self {
            r#type,
            identity: identity.into(),
            hashed,
            salt: None,
        }
    }
}

// NOTE This is already tested in assertion.rs

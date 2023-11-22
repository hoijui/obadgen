// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// The type of verification method.
/// Supported values for single assertion verification are `HostedBadge` and `SignedBadge`
/// (aliases in [context](
/// http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/v2/context.json)
/// are available: `hosted` and `signed`).
/// For instances used in `Profile`s,
/// the type `VerificationObject` should be used.
#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum ObjType {
    #[default]
    #[serde(alias = "hosted")]
    HostedBadge,
    #[serde(alias = "signed")]
    SignedBadge {
        /// The (HTTP) `id` of the key used to sign the `Assertion`.
        /// If not present, verifiers will check public key(s) declared in the referenced issuer `Profile`.
        /// If a key is declared here, it must be authorized in the issuer `Profile` as well.
        /// `creator` is expected to be the dereferencable URI of a document that describes a `CryptographicKey`.
        #[serde(skip_serializing_if = "Option::is_none")]
        creator: Option<String>,
    },
    VerificationObject,
}

/// A collection of information allowing an inspector to verify an `Assertion`.
/// This is used as part of verification instructions in each `Assertion`,
/// but also as an instruction set in an issuer’s `Profile`
/// to describe verification instructions for `Assertion`s the issuer awards.
///
/// `HostedVerification` and `SignedVerification` are subclasses of `VerificationObject`.
/// Future subclasses may be developed
/// to indicate instructions for verifying `Assertion`s using different methods,
/// such as blockchain-based procedures.
#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
// #[serde(tag = "type")]
pub struct Obj {
    /// See [`ObjType`].
    #[serde(flatten)]
    #[serde(default)]
    #[builder(default)]
    pub r#type: ObjType,
    /// The `@id` of the property to be used for verification
    /// that an `Assertion` is within the allowed scope.
    /// Only `id` is supported.
    /// Verifiers will consider `id` the default value if `verificationProperty` is omitted
    /// or if an issuer `Profile` has no explicit verification instructions,
    /// so it may be safely omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    #[builder(setter(into))]
    pub verification_property: Option<String>,
    /// The URI fragment that the verification property must start with.
    /// Valid `Assertion`s must have an `id` within this scope.
    /// Multiple values allowed,
    /// and `Assertion`s will be considered valid if their `id` starts with one of these values.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    #[builder(setter(into))]
    pub starts_with: Option<String>,
    /// The [host registered name subcomponent](
    /// https://tools.ietf.org/html/rfc3986#section-3.2.2)
    /// of an allowed origin.
    /// Any given `id` URI will be considered valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[builder(default)]
    #[builder(setter(into))]
    pub allowed_origins: Option<String>,
}

impl Obj {
    #[must_use]
    pub const fn new(r#type: ObjType) -> Self {
        Self {
            r#type,
            verification_property: None,
            starts_with: None,
            allowed_origins: None,
        }
    }
}

// NOTE This is already tested in assertion.rs

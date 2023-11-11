// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

use biscuit::CompactJson;
use monostate::MustBe;
use serde::{Deserialize, Serialize};

use crate::serde_date_time::SerdeDateTime;
use crate::util::defaults::default_bool;

pub const RDF_CONTEXT: &str = "https://w3id.org/openbadges/v2";

pub trait Type
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

/// Converts a vector of strings (or rather display-ables)
/// into a single string, representing JSON code
/// that represents the whole vector.
#[must_use]
pub fn str_list_to_string_rep<S: AsRef<str> + Display>(list: &[S]) -> String {
    let mut s = String::new();
    let mut first = true;
    s.push('[');
    for item in list {
        if first {
            first = false;
        } else {
            s.push_str(", ");
        }
        s.push('"');
        s.push_str(item.as_ref());
        s.push('"');
    }
    s.push(']');
    s
}

/// A `Profile` is a collection of information
/// that describes the entity or organization using Open Badges.
/// Issuers must be represented as `Profiles`,
/// and recipients, endorsers, or other entities may also be represented using this vocabulary.
/// Each `Profile` that represents an `Issuer`
/// may be referenced in many `BadgeClass`es that it has defined.
/// Anyone can create and host an Issuer file to start issuing Open Badges.
/// Issuers may also serve as recipients of Open Badges,
/// often identified within an `Assertion` by specific properties,
/// like their url or contact email address.
/// An Issuer Profile is a subclass of the general `Profile`
/// with some additional requirements.
///
/// See the [definition (& example)](
/// http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#Profile).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Issuer")]
pub struct Issuer {
    #[serde(rename = "@context")]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// Valid JSON-LD representation of the `Issuer` or `Profile` type.
    /// In most cases, this will simply be the string Issuer
    /// or the more general `Profile`.
    /// An array including Issuer and other string elements
    /// that are either URLs or compact IRIs within the current context are allowed.
    pub r#type: MustBe!("Issuer"),
    /// Unique IRI for the Issuer/Profile file.
    /// Most platforms to date can only handle HTTP-based IRIs.
    pub id: String,
    /// The name of the entity or organization.
    pub name: Option<String>,
    /// The homepage or social media profile of the entity,
    /// whether individual or institutional.
    /// Should be a URL/URI Accessible via HTTP. (examples).
    pub url: Option<String>,
    /// A phone number for the entity.
    /// For maximum compatibility,
    /// the value should be expressed as a + and country code
    /// followed by the number with no spaces or other punctuation,
    /// like `+16175551212`
    /// ([E.164 format](http://en.wikipedia.org/wiki/E.164)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    /// A short description of the issuer entity or organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IRI or document representing an image of the issuer.
    /// This must be a PNG or SVG image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Contact address for the individual or organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The key(s) an issuer uses to sign Assertions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Instructions for how to verify Assertions published by this Profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<Verification>,
    /// HTTP URI of the Badge Revocation List used for marking revocation of signed badges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_list: Option<String>,
}

impl Issuer {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            context: MustBe!("https://w3id.org/openbadges/v2"),
            r#type: MustBe!("Issuer"),
            id: id.into(),
            name: None,
            url: None,
            telephone: None,
            description: None,
            image: None,
            email: None,
            public_key: None,
            verification: None,
            revocation_list: None,
        }
    }
}

impl Type for Issuer {}

/// A collection of information about the accomplishment recognized by the Open Badge.
/// Many assertions may be created corresponding to one `BadgeClass`.
/// See the [definition (& example)](
/// https://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#BadgeClass).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "BadgeClass")]
pub struct BadgeDefinition {
    #[serde(rename = "@context")]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// Valid JSON-LD representation of the `BadgeClass` type.
    /// In most cases, this will simply be the string `BadgeClass`.
    /// An array including `BadgeClass` and other string elements
    /// that are either URLs or compact IRIs within the current context are allowed.
    pub r#type: MustBe!("BadgeClass"),
    /// Unique IRI for the `BadgeClass`.
    /// Most platforms to date can only handle HTTP-based IRIs.
    /// Issuers using signed assertions are encouraged to publish `BadgeClasses` using HTTP IRIs
    /// but may instead use ephemeral `BadgeClasses`
    /// that use an id in another scheme such as `urn:uuid`.
    pub id: String,
    /// The name of the achievement.
    pub name: String,
    /// A short description of the achievement.
    pub description: String,
    /// IRI or document representing an image of the achievement.
    /// This must be a PNG or SVG image.
    pub image: String,
    /// URI or embedded criteria document describing how to earn the achievement.
    pub criteria: String,
    /// IRI or document describing the individual, entity, or organization
    /// that issued the badge.
    pub issuer: String,
    /// An object describing which objectives or educational standards
    /// this badge aligns to, if any.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    pub alignment: Vec<String>,
    /// Tags that describes the type of achievement.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    pub tags: Vec<String>,
}

impl BadgeDefinition {
    pub fn new<S: Into<String>>(
        id: S,
        name: S,
        description: S,
        image: S,
        criteria: S,
        issuer: S,
    ) -> Self {
        Self {
            context: MustBe!("https://w3id.org/openbadges/v2"),
            r#type: MustBe!("BadgeClass"),
            id: id.into(),
            name: name.into(),
            description: description.into(),
            image: image.into(),
            criteria: criteria.into(),
            issuer: issuer.into(),
            alignment: vec![],
            tags: vec![],
        }
    }
}

impl Type for BadgeDefinition {}

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
pub enum IdentityType {
    #[default]
    EMail,
    Url,
    Telephone,
    // TODO DID?
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
// #[serde(rename = "IdentityObject")]
pub struct Identity {
    pub r#type: IdentityType,
    /// Either the hash of the identity or the plaintext value.
    /// If it’s possible that the plaintext transmission and storage
    /// of the identity value would leak personally identifiable information
    /// where there is an expectation of privacy,
    /// it is strongly recommended that an `IdentityHash` be used.
    pub identity: String,
    /// Whether or not the identity value is hashed.
    pub hashed: bool,
    /// If the recipient is hashed,
    /// this should contain the string used to salt the hash.
    /// If this value is not provided,
    /// it should be assumed that the hash was not salted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
}

impl Identity {
    pub fn new<S: Into<String>, D: Into<SerdeDateTime>>(
        r#type: IdentityType,
        identity: S,
        hashed: bool,
    ) -> Self {
        Self {
            r#type,
            identity: identity.into(),
            hashed,
            salt: None,
        }
    }
}

/// The type of verification method.
/// Supported values for single assertion verification are `HostedBadge` and `SignedBadge`
/// (aliases in context are available: hosted and signed).
/// For instances used in `Profiles`,
/// the type `VerificationObject` should be used.
#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum VerificationType {
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

#[derive(Debug, Default, Eq, PartialEq, Clone, Serialize, Deserialize)]
// #[serde(tag = "type")]
pub struct Verification {
    // #[default]
    // HostedBadge,
    // #[serde(rename_all = "camelCase")]
    // SignedBadge { creator: String },
    #[serde(flatten)]
    pub r#type: VerificationType,
    /// The `@id` of the property to be used for verification
    /// that an `Assertion` is within the allowed scope.
    /// Only `id` is supported.
    /// Verifiers will consider `id` the default value if `verificationProperty` is omitted
    /// or if an issuer `Profile` has no explicit verification instructions,
    /// so it may be safely omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_property: Option<String>,
    /// The URI fragment that the verification property must start with.
    /// Valid `Assertions` must have an id within this scope.
    /// Multiple values allowed,
    /// and `Assertions` will be considered valid if their `id` starts with one of these values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_with: Option<String>,
    /// The [host registered name subcomponent](
    /// https://tools.ietf.org/html/rfc3986#section-3.2.2)
    /// of an allowed origin.
    /// Any given `id` URI will be considered valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<String>,
}

impl Verification {
    #[must_use]
    pub fn new(r#type: VerificationType) -> Self {
        Self {
            r#type,
            verification_property: None,
            starts_with: None,
            allowed_origins: None,
        }
    }
}

/// Assertions are representations of an awarded badge,
/// used to share information about a badge belonging to one earner.
/// Assertions are packaged for transmission as JSON objects
/// with a set of mandatory and optional properties.
/// Fields marked in bold letters are mandatory.
/// See the [definition (& example)](
/// https://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#Assertion).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Assertion")]
pub struct BadgeAssertion {
    #[serde(rename = "@context")]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// valid JSON-LD representation of the Assertion type.
    /// In most cases, this will simply be the string Assertion.
    /// An array including Assertion and other string elements
    /// that are either URLs or compact IRIs
    /// within the current context are allowed.
    pub r#type: MustBe!("Assertion"),
    /// Unique IRI for the Assertion. If using hosted verification,
    /// this should be the URI where the assertion is accessible.
    /// For signed Assertions,
    /// it is recommended to use a UUID in the `urn:uuid`` namespace.
    pub id: String,
    /// IRI or document that describes the type of badge being awarded.
    /// If an HTTP/HTTPS IRI The endpoint should be a BadgeClass.
    pub badge: String,
    /// The recipient of the achievement.
    pub recipient: Identity,
    /// Instructions for third parties to verify this assertion.
    /// (Alias "verify" may be used in context.)
    #[serde(alias = "verify")]
    pub verification: Verification,
    /// Timestamp of when the achievement was awarded.
    pub issued_on: SerdeDateTime,
    /// IRI or document representing an image representing this user’s achievement.
    /// This must be a PNG or SVG image,
    /// and should be prepared via the Baking specification.
    /// An 'unbaked' image for the badge is defined in the BadgeClass
    /// and should not be duplicated here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// IRI or document describing the work that the recipient did
    /// to earn the achievement.
    /// This can be a page that links out to other pages
    /// if linking directly to the work is infeasible.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default = "Vec::new")]
    pub evidence: Vec<String>,
    /// A narrative that connects multiple pieces of evidence.
    /// Likely only present at this location if evidence is a multi-value array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narrative: Option<String>,
    /// If the achievement has some notion of expiry,
    /// this indicates a timestamp when a badge should no longer be considered valid.
    /// After this time, the badge should be considered expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<SerdeDateTime>,
    /// Defaults to false if Assertion is not referenced
    /// from a revokedAssertions list and may be omitted.
    /// See RevocationList.
    /// If revoked is true,
    /// only revoked and id are required properties,
    /// and many issuers strip a hosted Assertion down
    /// to only those properties when revoked.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    // #[serde(default)] // This is also `false`
    #[serde(default = "default_bool::<false>")]
    pub revoked: bool,
    /// Optional published reason for revocation, if revoked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
}

impl BadgeAssertion {
    pub fn new<S: Into<String>, D: Into<SerdeDateTime>>(
        id: S,
        badge: S,
        recipient: Identity,
        verification: Verification,
        issued_on: D,
    ) -> Self {
        Self {
            context: MustBe!("https://w3id.org/openbadges/v2"),
            r#type: MustBe!("Assertion"),
            id: id.into(),
            badge: badge.into(),
            recipient,
            verification,
            issued_on: issued_on.into(),
            image: None,
            evidence: vec![],
            narrative: None,
            expires: None,
            revoked: false,
            revocation_reason: None,
        }
    }
}

impl CompactJson for BadgeAssertion {}
impl Type for BadgeAssertion {}

/// Alias for the [Key](https://web-payments.org/vocabs/security#Key) class
/// from the W3C Web Payments Community Group Security Vocabulary.
/// A `CryptographicKey` document identifies
/// and describes a Key used for signing Open Badges documents.
///
/// For best compatibility with verification procedures,
/// the `Profile` should be hosted at its HTTPS `id`
/// and should identify a `publicKey` by the HTTPS `id` of a `CryptographicKey` document
/// that identifies its issuer by the issuer’s `id` using the `owner` property.
/// This allows convenient and robust usage of these `id`s
/// to identify both the issuer and the key used.
/// See the [definition (& example)](
/// https://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#CryptographicKey).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Assertion")]
pub struct CryptographicKey {
    #[serde(rename = "@context")]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// CryptographicKey
    pub r#type: MustBe!("CryptographicKey"),
    /// The identifier for the key. Most platforms only support HTTP(s) identifiers.
    pub id: String,
    /// The identifier for the Profile that owns this key.
    /// There should be a two-way connection between this Profile
    /// and the CryptographicKey through the `owner` and `publicKey` properties.
    pub owner: String,
    /// The PEM key encoding is a widely-used method to express public keys,
    /// compatible with almost every Secure Sockets Layer library implementation.
    pub public_key_pem: String,
}

impl CryptographicKey {
    pub fn new<S: Into<String>>(id: S, owner: S, public_key_pem: S) -> Self {
        Self {
            context: MustBe!("https://w3id.org/openbadges/v2"),
            r#type: MustBe!("CryptographicKey"),
            id: id.into(),
            owner: owner.into(),
            public_key_pem: public_key_pem.into(),
        }
    }
}

impl Type for CryptographicKey {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{box_err::BoxResult, constants};
    use chrono::DateTime;

    #[test]
    fn test_str_list_to_string_rep() -> BoxResult<()> {
        assert_eq!(
            str_list_to_string_rep(&Vec::<&str>::new()),
            r#"[]"#.to_owned()
        );
        assert_eq!(
            str_list_to_string_rep(&["subscriber"].to_vec()),
            r#"["subscriber"]"#.to_owned()
        );
        assert_eq!(
            str_list_to_string_rep(&["subscriber", "reader"].to_vec()),
            r#"["subscriber", "reader"]"#.to_owned()
        );
        assert_eq!(
            str_list_to_string_rep(&["subscriber", "reader", "architect"].to_vec()),
            r#"["subscriber", "reader", "architect"]"#.to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_issuer_1() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Issuer::new("AAA");
        obj.name = Some("BBB".to_string());
        obj.url = Some("CCC".to_string());
        let json_ld = serde_json::to_string_pretty(&obj)?;
        assert_eq!(
            json_ld,
            r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Issuer",
  "id": "AAA",
  "name": "BBB",
  "url": "CCC"
}"#
            .to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_issuer_2() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Issuer::new("http://thejeshgn.github.io/openbadge/issuer-organization.json");
        obj.name = Some("Thejesh GN".to_string());
        obj.url = Some("https://thejeshgn.com".to_string());
        let json_ld = serde_json::to_string_pretty(&obj)?;
        assert_eq!(
            json_ld,
            r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Issuer",
  "id": "http://thejeshgn.github.io/openbadge/issuer-organization.json",
  "name": "Thejesh GN",
  "url": "https://thejeshgn.com"
}"#
            .to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_issuer_3() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Issuer::new("http://abc.de/org.json");
        obj.name = Some("John Doe".to_string());
        obj.url = Some("https://abc.de/".to_string());
        let json_ld = serde_json::to_string_pretty(&obj)?;
        assert_eq!(
            json_ld,
            r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Issuer",
  "id": "http://abc.de/org.json",
  "name": "John Doe",
  "url": "https://abc.de/"
}"#
            .to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_issuer_4() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Issuer::new("http://abc.de/org.json");
        obj.name = Some("John Doe".to_string());
        obj.url = Some("https://abc.de/".to_string());
        obj.public_key = Some("http://abc.de/key.json".to_string());
        let json_ld = serde_json::to_string_pretty(&obj)?;
        assert_eq!(
            json_ld,
            r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Issuer",
  "id": "http://abc.de/org.json",
  "name": "John Doe",
  "url": "https://abc.de/",
  "publicKey": "http://abc.de/key.json"
}"#
            .to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_badge_definition() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = BadgeDefinition::new(
                "https://thejeshgn.github.io/openbadge/reader-badge.json",
                "Reader",
                "Reader of ThejeshGN.",
                "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png",
                "http://thejeshgn.com/subscribe",
                "http://thejeshgn.github.io/openbadge/issuer-organization.json",
            );
        obj.tags = ["subscriber".to_string(), "reader".to_string()].to_vec();
        let json_ld = serde_json::to_string_pretty(&obj)?;
        assert_eq!(
            json_ld,
r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "BadgeClass",
  "id": "https://thejeshgn.github.io/openbadge/reader-badge.json",
  "name": "Reader",
  "description": "Reader of ThejeshGN.",
  "image": "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png",
  "criteria": "http://thejeshgn.com/subscribe",
  "issuer": "http://thejeshgn.github.io/openbadge/issuer-organization.json",
  "tags": [
    "subscriber",
    "reader"
  ]
}"#.to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_badge_assertion() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = BadgeAssertion::new(
            "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
            "https://thejeshgn.github.io/openbadge/reader-badge.json",
            Identity {
                r#type: IdentityType::EMail,
                hashed: true,
                identity: "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130"
                    .to_string(),
                salt: None,
            },
            Verification::new(crate::open_badge::VerificationType::HostedBadge),
            DateTime::parse_from_rfc3339(constants::DT_PAST)?,
        );
        obj.expires = Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());
        let json_ld = serde_json::to_string_pretty(&obj)?;
        std::fs::write("simple_json_serde.json", &json_ld)?; // TODO HACK Remove this
                                                             // let simple_json_our_own = simple.serialize_to_json();
                                                             // std::fs::write("simple_json_our_own.json", &simple_json_our_own)?;
        assert_eq!(
            json_ld,
            r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Assertion",
  "id": "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
  "badge": "https://thejeshgn.github.io/openbadge/reader-badge.json",
  "recipient": {
    "type": "email",
    "identity": "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130",
    "hashed": true
  },
  "verification": {
    "type": "HostedBadge"
  },
  "issuedOn": "2022-06-17T23:59:59Z",
  "expires": "2099-06-30T23:59:59Z"
}"#
            .to_owned()
        );

        // assert_eq!(simple_json_our_own, simple_json_serde);

        Ok(())
    }

    #[test]
    fn test_cryptographic_key() -> Result<(), Box<dyn std::error::Error>> {
        let obj = CryptographicKey::new(
            "https://example.org/publicKey.json",
            "https://example.org/organization.json",
            "-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n",
        );
        let json_ld = obj.to_json_ld()?;
        assert_eq!(
            json_ld,
            r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "CryptographicKey",
  "id": "https://example.org/publicKey.json",
  "owner": "https://example.org/organization.json",
  "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n"
}"#
            .to_owned()
        );

        Ok(())
    }
}

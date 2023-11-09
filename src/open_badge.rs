// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;
use std::{fmt::Display, marker::PhantomData};

use chrono::{DateTime, FixedOffset, SecondsFormat, TimeZone};
use monostate::MustBe;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::{DeserializeOwned, Visitor, value::StringDeserializer}};

pub const RDF_CONTEXT: &str = "https://w3id.org/openbadges/v2";
macro_rules! rdf_context {
    () => {
        "https://w3id.org/openbadges/v2"
    };
}

pub trait Type {
    /// This generates Open Badge JSON-LD
    /// that represents the specific type.
    fn serialize_to_json(&self) -> String;
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

/// Open Badge 2.0 Issuer/Profile
///
/// - [Definition](http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#Profile)
/// - [Example](http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/examples/index.html#Issuer)
#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct Issuer<S: AsRef<str> + Display> {
    pub id: S,
    pub name: S,
    pub url: S,
    pub public_key: Option<S>,
}

impl<S: AsRef<str> + Display> Type for Issuer<S> {
    /// This generates Open Badge 2.0 compatible JSON-LD
    /// that represents a badge issuer (a person or organization);
    /// to then be hosted under the given URL.
    fn serialize_to_json(&self) -> String {
        let public_key_opt = if let Some(public_key_str) = &self.public_key {
            format!(",\n    \"publicKey\": \"{public_key_str}\"")
        } else {
            String::new()
        };
        format!(
            r#"{{
            "@context": "{RDF_CONTEXT}",
            "id": "{}",
            "type": "Issuer",
            "name": "{}",
            "url": "{}"{public_key_opt}
        }}"#,
            self.id, self.name, self.url,
        )
    }
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct BadgeDefinition<S: AsRef<str> + Display> {
    pub id: S,
    pub name: S,
    pub description: S,
    pub image_url: S,
    pub criteria: S,
    pub tags: Vec<S>,
    pub alignment: Vec<S>,
    pub issuer: S,
}

impl<S: AsRef<str> + Display> Type for BadgeDefinition<S> {
    /// This generates Open Badge 2.0 compatible JSON-LD
    /// that represents a badge definition;
    /// to then be hosted under the given URL.
    fn serialize_to_json(&self) -> String {
        format!(
            r#"{{
            "@context": "{RDF_CONTEXT}",
            "id": "{}",
            "type": "BadgeClass",
            "name": "{}",
            "description": "{}",
            "image": "{}",
            "criteria": "{}",
            "tags": {},
            "alignment": {},
            "issuer": "{}"
        }}"#,
            self.id,
            self.name,
            self.description,
            self.image_url,
            self.criteria,
            str_list_to_string_rep(&self.tags),
            str_list_to_string_rep(&self.alignment),
            self.issuer,
        )
    }
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
)]
pub struct SerdeDateTime(DateTime<FixedOffset>);

impl Serialize for SerdeDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}

impl From<DateTime<FixedOffset>> for SerdeDateTime {
    fn from(value: DateTime<FixedOffset>) -> Self {
        SerdeDateTime(value)
    }
}

struct SerdeDateTimeVisitor;

impl<'de> Visitor<'de> for SerdeDateTimeVisitor {
    type Value = SerdeDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an RFC3339 compliant date-time string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SerdeDateTime(DateTime::parse_from_rfc3339(value).map_err(|err| E::custom(err.to_string()))?))
    }
}

impl<'de> Deserialize<'de> for SerdeDateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // let value_str = Deserialize::deserialize::<&str>(deserializer);
        // let value_str = Deserialize::deserialize::<StringDeserializer<E: serde::de::Error>>(deserializer);
        // Ok(SerdeDateTime(DateTime::parse_from_rfc3339(value_str)?))
        // deserializer.deserialize_str(SerdeDateTimeVisitor)

        deserializer.deserialize_str(SerdeDateTimeVisitor)

        // ...deserialize implementation.
    }
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
// #[serde(default)]
#[serde(rename = "Recipient")]
pub struct BadgeRecipient {
    pub salt: Option<String>,
    pub hashed_email: String, // TODO Allow other ID types then email!
}

#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Verification {
    #[default]
    HostedBadge,
    SignedBadge,
}


#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[serde(rename = "Assertion")]
pub struct BadgeAssertion {
    #[serde(rename = "@context")]
    context: MustBe!("https://w3id.org/openbadges/v2"),
    r#type: MustBe!("Assertion"),
    pub id: String,
    pub badge_id: String,
    pub recipient: BadgeRecipient,
    pub verification: Verification,
    // #[serde(from = "SerdeDateTime")]
    // #[serde(into = "SerdeDateTime")]
    pub issued_on: DateTime<FixedOffset>,
    pub expires: SerdeDateTime,
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
#[serde(rename = "Assertion")]
pub struct BadgeAssertionVar<S: AsRef<str> + Display + Serialize> {
    pub id: S,
    pub badge_id: S,
    pub recipient_salt: Option<S>,
    pub recipient_hashed_email: S, // TODO Allow other ID types then email!
    pub verification_public_key: Option<S>,
    // #[serde(from = "SerdeDateTime")]
    // #[serde(into = "SerdeDateTime")]
    pub issued_on: SerdeDateTime,
    pub expires: SerdeDateTime,
}

impl<S: AsRef<str> + Display + Serialize + DeserializeOwned> biscuit::CompactJson for BadgeAssertion<S> {}

// impl BadgeAssertion<&str> {
//     pub fn to_deserializable(&self) -> BadgeAssertion<String> {
//         BadgeAssertion {
//             sssss: MustBe!("Assertion"),
//             id: self.id.to_string(),
//             badge_id: self.badge_id.to_string(),
//             recipient_salt: self.recipient_salt.map(ToString::to_string),
//             recipient_hashed_email: self.recipient_hashed_email.to_string(),
//             verification_public_key: self.verification_public_key.map(ToString::to_string),
//             issued_on: self.issued_on.clone(),
//             expires: self.expires.clone(),
//         }
//     }
// }

impl<S: AsRef<str> + Display + Serialize> Type for BadgeAssertion<S> {
    /// This generates Open Badge 2.0 compatible JSON-LD
    /// that represents an issue of a badge for an individual.
    fn serialize_to_json(&self) -> String {
        let verification_type = if self.verification_public_key.is_some() {
            // alternative: "signed"
            "SignedBadge"
        } else {
            // alternative: "hosted"
            "HostedBadge"
        };
        let verification_creator_opt = if let Some(public_key_str) = &self.verification_public_key {
            format!(",\n    \"creator\": \"{public_key_str}\"")
        } else {
            String::new()
        };
        let recipient_salt_opt = if let Some(recipient_salt_str) = &self.recipient_salt {
            format!(",\n    \"salt\": \"{recipient_salt_str}\"")
        } else {
            String::new()
        };
        format!(
            r#"{{
            "@context": "{RDF_CONTEXT}",
            "type": "Assertion",
            "id": "{}",
            "recipient":
            {{
                "type": "email",
                "hashed": true{recipient_salt_opt},
                "identity": "{}"
            }},
            "badge": "{}",
            "verification":
            {{
                "type": "{verification_type}"{verification_creator_opt}
            }},
            "issuedOn": "{}",
            "expires": "{}"
        }}"#,
            self.id,
            self.recipient_hashed_email,
            self.badge_id,
            self.issued_on.0.to_rfc3339_opts(SecondsFormat::Secs, true),
            self.expires.0.to_rfc3339_opts(SecondsFormat::Secs, true),
        )
    }
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    Clone,
    Serialize,
    Deserialize,
)]
pub struct CryptographicKey<S: AsRef<str> + Display> {
    pub id: S,
    pub owner_id: S,
    pub public_key_pem: S,
}

impl<S: AsRef<str> + Display> Type for CryptographicKey<S> {
    /// This generates Open Badge 2.0 compatible JSON-LD
    /// that represents a (JWS) cryptographic key for validating an assertion
    /// that uses ["signed" hosting](http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#CryptographicKey)
    /// ([example](http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/examples/index.html#CryptographicKey)).
    fn serialize_to_json(&self) -> String {
        format!(
            r#"{{
            "@context": "{RDF_CONTEXT}",
            "type": "CryptographicKey",
            "id": "{}",
            "owner": "{}",
            "publicKeyPem": "{}"
        }}"#,
            self.id, self.owner_id, self.public_key_pem
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::box_err::BoxResult;
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
        assert_eq!(
            Issuer {
                id: "AAA",
                name: "BBB",
                url: "CCC",
                public_key: None,
            }
            .serialize_to_json(),
            r#"{
            "@context": "https://w3id.org/openbadges/v2",
            "id": "AAA",
            "type": "Issuer",
            "name": "BBB",
            "url": "CCC"
        }"#
            .to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_issuer_2() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Issuer {
                id: "http://thejeshgn.github.io/openbadge/issuer-organization.json",
                name: "Thejesh GN",
                url: "https://thejeshgn.com",
                public_key: None,
            }
            .serialize_to_json(),
            r#"{
            "@context": "https://w3id.org/openbadges/v2",
            "id": "http://thejeshgn.github.io/openbadge/issuer-organization.json",
            "type": "Issuer",
            "name": "Thejesh GN",
            "url": "https://thejeshgn.com"
        }"#
            .to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_issuer_3() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            Issuer {
                id: "http://abc.de/org.json",
                name: "John Doe",
                url: "https://abc.de/",
                public_key: None,
            }
            .serialize_to_json(),
            r#"{
            "@context": "https://w3id.org/openbadges/v2",
            "id": "http://abc.de/org.json",
            "type": "Issuer",
            "name": "John Doe",
            "url": "https://abc.de/"
        }"#
            .to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_badge_definition() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            BadgeDefinition {
                id: "https://thejeshgn.github.io/openbadge/reader-badge.json",
                name: "Reader",
                description: "Reader of ThejeshGN.",
                image_url: "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png",
                criteria: "http://thejeshgn.com/subscribe",
                tags: ["subscriber", "reader"].to_vec(),
                alignment: [ ].to_vec(),
                issuer: "http://thejeshgn.github.io/openbadge/issuer-organization.json",
            }.serialize_to_json(),
        r#"{
            "@context": "https://w3id.org/openbadges/v2",
            "id": "https://thejeshgn.github.io/openbadge/reader-badge.json",
            "type": "BadgeClass",
            "name": "Reader",
            "description": "Reader of ThejeshGN.",
            "image": "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png",
            "criteria": "http://thejeshgn.com/subscribe",
            "tags": ["subscriber", "reader"],
            "alignment": [],
            "issuer": "http://thejeshgn.github.io/openbadge/issuer-organization.json"
        }"#.to_owned()
        );
        Ok(())
    }

    #[test]
    fn test_badge_assertion() -> Result<(), Box<dyn std::error::Error>> {
        let simple = BadgeAssertion {
            id: "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
            badge_id: "https://thejeshgn.github.io/openbadge/reader-badge.json",
            recipient_salt: None,
            recipient_hashed_email:
                "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130",
            verification_public_key: None,
            issued_on: SerdeDateTime(DateTime::parse_from_rfc3339("2022-06-17T23:59:59Z")?),
            expires: SerdeDateTime(DateTime::parse_from_rfc3339("2030-06-30T23:59:59Z")?),
        };

        assert_eq!(
            simple.serialize_to_json(),
        r#"{
            "@context": "https://w3id.org/openbadges/v2",
            "type": "Assertion",
            "id": "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
            "recipient":
            {
                "type": "email",
                "hashed": true,
                "identity": "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130"
            },
            "badge": "https://thejeshgn.github.io/openbadge/reader-badge.json",
            "verification":
            {
                "type": "HostedBadge"
            },
            "issuedOn": "2022-06-17T23:59:59Z",
            "expires": "2030-06-30T23:59:59Z"
        }"#.to_owned()
        );

        let simple_json_serde = serde_json::to_string_pretty(&simple)?;
        let simple_json_our_own = simple.serialize_to_json();

        std::fs::write("simple_json_serde.json", &simple_json_serde)?;
        std::fs::write("simple_json_our_own.json", &simple_json_our_own)?;

        assert_eq!(simple_json_our_own, simple_json_serde);

        Ok(())
    }

    #[test]
    fn test_cryptographic_key() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            CryptographicKey {
                id: "https://example.org/publicKey.json",
                owner_id: "https://example.org/organization.json",
                public_key_pem: r#"-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n"#,
            }.serialize_to_json(),
        r#"{
            "@context": "https://w3id.org/openbadges/v2",
            "type": "CryptographicKey",
            "id": "https://example.org/publicKey.json",
            "owner": "https://example.org/organization.json",
            "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n"
        }"#.to_owned()
        );

        Ok(())
    }
}

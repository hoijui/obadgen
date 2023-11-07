// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, TimeZone};

pub const RDF_CONTEXT: &str = "https://w3id.org/openbadges/v2";

pub trait Type {
    /// This generates Open Badge JSON-LD
    /// that represents the specific type.
    fn serialize(&self) -> String;
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
    fn serialize(&self) -> String {
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
    fn serialize(&self) -> String {
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

pub struct BadgeAssertion<S: AsRef<str> + Display, Tz1: TimeZone, Tz2: TimeZone> {
    pub id: S,
    pub badge_id: S,
    pub recipient_salt: Option<S>,
    pub recipient_hashed_email: S, // TODO Allow other ID types then email!
    pub verification_public_key: Option<S>,
    pub issued_on: DateTime<Tz1>,
    pub expires: DateTime<Tz2>,
}

impl<S: AsRef<str> + Display, Tz1: TimeZone, Tz2: TimeZone> Type for BadgeAssertion<S, Tz1, Tz2> {
    /// This generates Open Badge 2.0 compatible JSON-LD
    /// that represents an issue of a badge for an individual.
    fn serialize(&self) -> String {
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
            self.issued_on.to_rfc3339_opts(SecondsFormat::Secs, true),
            self.expires.to_rfc3339_opts(SecondsFormat::Secs, true),
        )
    }
}

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
    fn serialize(&self) -> String {
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
            .serialize(),
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
            .serialize(),
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
            .serialize(),
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
            }.serialize(),
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
        assert_eq!(
            BadgeAssertion {
                id: "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
                badge_id: "https://thejeshgn.github.io/openbadge/reader-badge.json",
                recipient_salt: None,
                recipient_hashed_email:
                    "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130",
                verification_public_key: None,
                issued_on: DateTime::parse_from_rfc3339("2022-06-17T23:59:59Z")?,
                expires: DateTime::parse_from_rfc3339("2030-06-30T23:59:59Z")?,
            }.serialize(),
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
        Ok(())
    }

    #[test]
    fn test_cryptographic_key() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            CryptographicKey {
                id: "https://example.org/publicKey.json",
                owner_id: "https://example.org/organization.json",
                public_key_pem: r#"-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n"#,
            }.serialize(),
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

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

use chrono::{DateTime, SecondsFormat, TimeZone};

pub const RDF_CONTEXT: &str = "https://w3id.org/openbadges/v2";

/// Converts a vector of strings (or rather display-ables)
/// into a single string, representing JSON code
/// that represents the whole vector.
///
/// for example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use obadgen::open_badge::str_list_to_string_rep;
/// assert_eq!(
///     str_list_to_string_rep(Vec::<&str>::new()),
///     r#"[]"#.to_owned()
/// );
/// assert_eq!(
///     str_list_to_string_rep(["subscriber"].to_vec()),
///     r#"["subscriber"]"#.to_owned()
/// );
/// assert_eq!(
///     str_list_to_string_rep(["subscriber", "reader"].to_vec()),
///     r#"["subscriber", "reader"]"#.to_owned()
/// );
/// assert_eq!(
///     str_list_to_string_rep(["subscriber", "reader", "architect"].to_vec()),
///     r#"["subscriber", "reader", "architect"]"#.to_owned()
/// );
/// # Ok(())
/// # }
/// ```
#[must_use]
pub fn str_list_to_string_rep<S: AsRef<str> + Display>(list: Vec<S>) -> String {
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

/// This generates Open Badge 2.0 compatible JSON-LD
/// that represents a badge issuer (a person or organization);
/// to then be hosted under the given URL.
///
/// for example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use obadgen::open_badge::create_issuer;
/// assert_eq!(
///     create_issuer(
///         "AAA",
///         "BBB",
///         "CCC",
///         None,
///     ),
///     r#"{
///         "@context": "https://w3id.org/openbadges/v2",
///         "id": "AAA",
///         "type": "Issuer",
///         "name": "BBB",
///         "url": "CCC"
///     }"#.to_owned()
/// );
/// assert_eq!(
///     create_issuer(
///         "http://thejeshgn.github.io/openbadge/issuer-organization.json",
///         "Thejesh GN",
///         "https://thejeshgn.com",
///         None,
///     ),
///     r#"{
///         "@context": "https://w3id.org/openbadges/v2",
///         "id": "http://thejeshgn.github.io/openbadge/issuer-organization.json",
///         "type": "Issuer",
///         "name": "Thejesh GN",
///         "url": "https://thejeshgn.com"
///     }"#.to_owned()
/// );
/// assert_eq!(
///     create_issuer(
///         "http://abc.de/org.json",
///         "John Doe",
///         "https://abc.de/",
///         None,
///     ),
///     r#"{
///         "@context": "https://w3id.org/openbadges/v2",
///         "id": "http://abc.de/org.json",
///         "type": "Issuer",
///         "name": "John Doe",
///         "url": "https://abc.de/"
///     }"#.to_owned()
/// );
/// # Ok(())
/// # }
/// ```
pub fn create_issuer<S: AsRef<str> + Display>(
    id: S,
    name: S,
    url: S,
    public_key: Option<S>,
) -> String {
    let public_key_opt = if let Some(public_key_str) = public_key {
        format!(",\n    \"publicKey\": \"{public_key_str}\"")
    } else {
        String::new()
    };
    format!(
        r#"{{
        "@context": "{RDF_CONTEXT}",
        "id": "{id}",
        "type": "Issuer",
        "name": "{name}",
        "url": "{url}"{public_key_opt}
    }}"#
    )
}

/// This generates Open Badge 2.0 compatible JSON-LD
/// that represents a badge definition;
/// to then be hosted under the given URL.
///
/// for example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use obadgen::open_badge::create_badge_definition;
/// assert_eq!(
///     create_badge_definition(
///         "https://thejeshgn.github.io/openbadge/reader-badge.json",
///         "Reader",
///         "Reader of ThejeshGN.",
///         "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png",
///         "http://thejeshgn.com/subscribe",
///         ["subscriber", "reader"].to_vec(),
///         [ ].to_vec(),
///         "http://thejeshgn.github.io/openbadge/issuer-organization.json",
///     ),
///     r#"{
///         "@context": "https://w3id.org/openbadges/v2",
///         "id": "https://thejeshgn.github.io/openbadge/reader-badge.json",
///         "type": "BadgeClass",
///         "name": "Reader",
///         "description": "Reader of ThejeshGN.",
///         "image": "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png",
///         "criteria": "http://thejeshgn.com/subscribe",
///         "tags": ["subscriber", "reader"],
///         "alignment": [],
///         "issuer": "http://thejeshgn.github.io/openbadge/issuer-organization.json"
///     }"#.to_owned()
/// );
/// # Ok(())
/// # }
/// ```
pub fn create_badge_definition<S: AsRef<str> + Display>(
    id: S,
    name: S,
    description: S,
    image_url: S,
    criteria: S,
    tags: Vec<S>,
    alignment: Vec<S>,
    issuer: S,
) -> String {
    format!(
        r#"{{
        "@context": "{RDF_CONTEXT}",
        "id": "{id}",
        "type": "BadgeClass",
        "name": "{name}",
        "description": "{description}",
        "image": "{image_url}",
        "criteria": "{criteria}",
        "tags": {},
        "alignment": {},
        "issuer": "{issuer}"
    }}"#,
        str_list_to_string_rep(tags),
        str_list_to_string_rep(alignment)
    )
}

/// This generates Open Badge 2.0 compatible JSON-LD
/// that represents an issue of a badge for an individual.
///
/// for example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use obadgen::open_badge::create_badge_assertion;
/// # use chrono::DateTime;
/// assert_eq!(
///     create_badge_assertion(
///         "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
///         "https://thejeshgn.github.io/openbadge/reader-badge.json",
///         None,
///         "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130",
///         None,
///         &DateTime::parse_from_rfc3339("2022-06-17T23:59:59Z")?,
///         &DateTime::parse_from_rfc3339("2030-06-30T23:59:59Z")?,
///     ),
///     r#"{
///         "@context": "https://w3id.org/openbadges/v2",
///         "type": "Assertion",
///         "id": "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
///         "recipient":
///         {
///             "type": "email",
///             "hashed": true,
///             "identity": "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130"
///         },
///         "badge": "https://thejeshgn.github.io/openbadge/reader-badge.json",
///         "verification":
///         {
///             "type": "HostedBadge"
///         },
///         "issuedOn": "2022-06-17T23:59:59Z",
///         "expires": "2030-06-30T23:59:59Z"
///     }"#.to_owned()
/// );
/// # Ok(())
/// # }
/// ```
pub fn create_badge_assertion<S: AsRef<str> + Display, Tz1: TimeZone, Tz2: TimeZone>(
    id: S,
    badge_id: S,
    recipient_salt: Option<S>,
    recipient_hashed_email: S, // TODO Allow other ID types then email!
    verification_public_key: Option<S>,
    issued_on: &DateTime<Tz1>,
    expires: &DateTime<Tz2>,
) -> String {
    let verification_type = if verification_public_key.is_some() {
        // alternative: "signed"
        "SignedBadge"
    } else {
        // alternative: "hosted"
        "HostedBadge"
    };
    let verification_creator_opt = if let Some(public_key_str) = verification_public_key {
        format!(",\n    \"creator\": \"{public_key_str}\"")
    } else {
        String::new()
    };
    let recipient_salt_opt = if let Some(recipient_salt_str) = recipient_salt {
        format!(",\n    \"salt\": \"{recipient_salt_str}\"")
    } else {
        String::new()
    };
    format!(
        r#"{{
        "@context": "{RDF_CONTEXT}",
        "type": "Assertion",
        "id": "{id}",
        "recipient":
        {{
            "type": "email",
            "hashed": true{recipient_salt_opt},
            "identity": "{recipient_hashed_email}"
        }},
        "badge": "{badge_id}",
        "verification":
        {{
            "type": "{verification_type}"{verification_creator_opt}
        }},
        "issuedOn": "{}",
        "expires": "{}"
    }}"#,
        issued_on.to_rfc3339_opts(SecondsFormat::Secs, true),
        expires.to_rfc3339_opts(SecondsFormat::Secs, true),
    )
}

/// This generates Open Badge 2.0 compatible JSON-LD
/// that represents a (JWS) cryptographic key for validating an assertion
/// that uses ["signed" hosting](http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#CryptographicKey)
/// ([example](http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/examples/index.html#CryptographicKey)).
///
/// for example:
///
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use obadgen::open_badge::create_cryptographic_key;
/// assert_eq!(
///     create_cryptographic_key(
///         "https://example.org/publicKey.json",
///         "https://example.org/organization.json",
///         r#"-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n"#,
///     ),
///     r#"{
///         "@context": "https://w3id.org/openbadges/v2",
///         "type": "CryptographicKey",
///         "id": "https://example.org/publicKey.json",
///         "owner": "https://example.org/organization.json",
///         "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n"
///     }"#.to_owned()
/// );
/// # Ok(())
/// # }
/// ```
pub fn create_cryptographic_key<S: AsRef<str> + Display>(
    id: S,
    owner_id: S,
    public_key_pem: S,
) -> String {
    format!(
        r#"{{
        "@context": "{RDF_CONTEXT}",
        "type": "CryptographicKey",
        "id": "{id}",
        "owner": "{owner_id}",
        "publicKeyPem": "{public_key_pem}"
    }}"#
    )
}

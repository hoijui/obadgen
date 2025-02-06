// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use monostate::MustBe;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Assertion")]
pub struct Obj {
    #[serde(rename = "@context")]
    #[builder(default = MustBe!("https://w3id.org/openbadges/v2"))]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// `CryptographicKey`
    #[builder(default = MustBe!("CryptographicKey"))]
    pub r#type: MustBe!("CryptographicKey"),
    /// The identifier for the key. Most platforms only support HTTP(s) identifiers.
    #[builder(setter(into))]
    pub id: String,
    /// The identifier for the Profile that owns this key.
    /// There should be a two-way connection between this Profile
    /// and the `CryptographicKey` through the `owner` and `publicKey` properties.
    #[builder(setter(into))]
    pub owner: String,
    /// The PEM key encoding is a widely-used method to express public keys,
    /// compatible with almost every Secure Sockets Layer library implementation.
    #[builder(setter(into))]
    pub public_key_pem: String,
}

impl Obj {
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

impl super::ToJsonLd for Obj {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ToJsonLd;

    const EXP_JSON_LD_SIMPLE: &str = r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "CryptographicKey",
  "id": "https://example.org/publicKey.json",
  "owner": "https://example.org/organization.json",
  "publicKeyPem": "-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n"
}"#;

    #[test]
    fn test_new_simple() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::new(
            "https://example.org/publicKey.json",
            "https://example.org/organization.json",
            "-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n",
        );
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);

        Ok(())
    }

    #[test]
    fn test_builder_simple() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder()
            .id("https://example.org/publicKey.json")
            .owner("https://example.org/organization.json")
            .public_key_pem(
                "-----BEGIN PUBLIC KEY-----\nMIIBG0BA...OClDQAB\n-----END PUBLIC KEY-----\n",
            )
            .build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);

        Ok(())
    }

    #[test]
    fn test_rawe_key() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::new(
            "https://example.org/publicKey.json",
            "https://example.org/organization.json",
            "-----BEGIN PUBLIC KEY-----
MIIBG0BA...OClDQAB
-----END PUBLIC KEY-----
",
        );
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);

        Ok(())
    }
}

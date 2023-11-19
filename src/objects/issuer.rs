// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use monostate::MustBe;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::verification::Obj as Verification;

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
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Issuer")]
pub struct Obj {
    #[serde(rename = "@context")]
    #[builder(default = MustBe!("https://w3id.org/openbadges/v2"))]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// Valid JSON-LD representation of the `Issuer` or `Profile` type.
    /// In most cases, this will simply be the string Issuer
    /// or the more general `Profile`.
    /// An array including Issuer and other string elements
    /// that are either URLs or compact IRIs within the current context are allowed.
    #[builder(default = MustBe!("Issuer"))]
    pub r#type: MustBe!("Issuer"),
    /// Unique IRI for the Issuer/Profile file.
    /// Most platforms to date can only handle HTTP-based IRIs.
    #[builder(setter(into))]
    pub id: String,
    /// The name of the entity or organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub name: Option<String>,
    /// The homepage or social media profile of the entity,
    /// whether individual or institutional.
    /// Should be a URL/URI Accessible via HTTP. (examples).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub url: Option<String>,
    /// A phone number for the entity.
    /// For maximum compatibility,
    /// the value should be expressed as a + and country code
    /// followed by the number with no spaces or other punctuation,
    /// like `+16175551212`
    /// ([E.164 format](http://en.wikipedia.org/wiki/E.164)).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub telephone: Option<String>,
    /// A short description of the issuer entity or organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub description: Option<String>,
    /// IRI or document representing an image of the issuer.
    /// This must be a PNG or SVG image.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub image: Option<String>,
    /// Contact address for the individual or organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub email: Option<String>,
    /// The key(s) an issuer uses to sign `Assertion`s.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub public_key: Option<String>,
    /// Instructions for how to verify `Assertion`s published by this Profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    pub verification: Option<Verification>,
    /// HTTP URI of the Badge Revocation List used for marking revocation of signed badges.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub revocation_list: Option<String>,
}

impl Obj {
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

impl super::ToJsonLd for Obj {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ToJsonLd;

    const EXP_JSON_LD_SIMPLE: &str = r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Issuer",
  "id": "http://abc.de/org.json",
  "name": "John Doe",
  "url": "https://abc.de/"
}"#;

    const EXP_JSON_LD_WITH_KEY: &str = r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Issuer",
  "id": "http://abc.de/org.json",
  "name": "John Doe",
  "url": "https://abc.de/",
  "publicKey": "http://abc.de/key.json"
}"#;

    #[test]
    fn test_new_simple() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Obj::new("http://abc.de/org.json");
        obj.name = Some("John Doe".to_string());
        obj.url = Some("https://abc.de/".to_string());
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);
        Ok(())
    }

    #[test]
    fn test_new_with_key() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Obj::new("http://abc.de/org.json");
        obj.name = Some("John Doe".to_string());
        obj.url = Some("https://abc.de/".to_string());
        obj.public_key = Some("http://abc.de/key.json".to_string());
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_WITH_KEY);
        Ok(())
    }

    #[test]
    fn test_builder_simple() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder()
            .id("http://abc.de/org.json")
            .name("John Doe")
            .url("https://abc.de/")
            .build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);
        Ok(())
    }

    #[test]
    fn test_builder_with_key() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder()
            .id("http://abc.de/org.json")
            .name("John Doe")
            .url("https://abc.de/")
            .public_key("http://abc.de/key.json")
            .build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_WITH_KEY);
        Ok(())
    }
}

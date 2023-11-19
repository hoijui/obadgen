// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use monostate::MustBe;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// A collection of information about the accomplishment recognized by the Open Badge.
/// Many assertions may be created corresponding to one `BadgeClass`.
/// See the [definition (& example)](
/// https://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#BadgeClass).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "BadgeClass")]
pub struct Obj {
    #[serde(rename = "@context")]
    #[builder(default = MustBe!("https://w3id.org/openbadges/v2"))]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// Valid JSON-LD representation of the `BadgeClass` type.
    /// In most cases, this will simply be the string `BadgeClass`.
    /// An array including `BadgeClass` and other string elements
    /// that are either URLs or compact IRIs within the current context are allowed.
    #[builder(default = MustBe!("BadgeClass"))]
    pub r#type: MustBe!("BadgeClass"),
    /// Unique IRI for the `BadgeClass`.
    /// Most platforms to date can only handle HTTP-based IRIs.
    /// Issuers using signed assertions are encouraged to publish `BadgeClasses` using HTTP IRIs
    /// but may instead use ephemeral `BadgeClasses`
    /// that use an id in another scheme such as `urn:uuid`.
    #[builder(setter(into))]
    pub id: String,
    /// The name of the achievement.
    #[builder(setter(into))]
    pub name: String,
    /// A short description of the achievement.
    #[builder(setter(into))]
    pub description: String,
    /// IRI or document representing an image of the achievement.
    /// This must be a PNG or SVG image.
    #[builder(setter(into))]
    pub image: String,
    /// URI or embedded criteria document describing how to earn the achievement.
    #[builder(setter(into))]
    pub criteria: String,
    /// IRI or document describing the individual, entity, or organization
    /// that issued the badge.
    #[builder(setter(into))]
    pub issuer: String,
    /// An object describing which objectives or educational standards
    /// this badge aligns to, if any.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[builder(default)]
    #[builder(setter(into))]
    pub alignment: Vec<String>,
    /// Tags that describes the type of achievement.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[builder(default)]
    #[builder(setter(into))]
    pub tags: Vec<String>,
}

impl Obj {
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

impl super::ToJsonLd for Obj {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::ToJsonLd;

    const EXP_JSON_LD_SIMPLE: &str = r#"{
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
}"#;

    #[test]
    fn test_new_simple() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Obj::new(
                "https://thejeshgn.github.io/openbadge/reader-badge.json",
                "Reader",
                "Reader of ThejeshGN.",
                "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png",
                "http://thejeshgn.com/subscribe",
                "http://thejeshgn.github.io/openbadge/issuer-organization.json",
            );
        obj.tags = ["subscriber".to_string(), "reader".to_string()].to_vec();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);
        Ok(())
    }

    #[test]
    fn test_builder_simple() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder()
            .id("https://thejeshgn.github.io/openbadge/reader-badge.json")
            .name("Reader")
            .description("Reader of ThejeshGN.")
            .image("https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png")
            .criteria("http://thejeshgn.com/subscribe")
            .issuer("http://thejeshgn.github.io/openbadge/issuer-organization.json")
            .tags(["subscriber".to_string(), "reader".to_string()])
            .build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);
        Ok(())
    }
}

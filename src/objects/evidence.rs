// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Descriptive metadata about evidence related to the issuance of an `Assertion`.
/// Each instance of the `Evidence` class present
/// in an `Assertion` corresponds to one entity,
/// though a single entry can describe a set of items collectively.
/// There may be multiple evidence entries referenced from an Assertion.
/// The narrative property is also in scope of the `Assertion` class
/// to provide an overall description of the achievement
/// related to the badge in rich text.
/// It is used here to provide a narrative of achievement
/// of the specific entity described.
///
/// If both the `description` and `narrative` properties are present,
/// displayers can assume the `narrative` value goes into more detail
/// and is not simply a recapitulation of `description`.
///
/// For evidence that is ephemeral
/// or completely described within an `Assertion` via use of the `Evidence` class,
/// if it is necessary to identify this evidence piece uniquely in an overall narrative,
/// an `id` of type `urn:uuid` or otherwise outside the HTTP scheme may be used,
/// but displayers may have less success displaying this usage meaningfully.
///
/// See the [definition (& example)](
/// https://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#Evidence).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Evidence")]
pub struct Obj {
    /// Defaults to `Evidence`.
    ///
    /// - JSON-LD Type (Multiple values allowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default=Some("Evidence".to_string()))]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub r#type: Option<String>,
    /// The URI of a webpage presenting evidence of achievement.
    ///
    /// - IRI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub id: Option<String>,
    /// A narrative that describes the evidence and process of achievement
    /// that led to an `Assertion`.
    ///
    /// - Text or Markdown Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub narrative: Option<String>,
    /// A descriptive title of the evidence.
    ///
    /// - Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub name: Option<String>,
    /// A longer description of the evidence.
    ///
    /// - Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub description: Option<String>,
    /// A string that describes the type of evidence.
    /// For example, Poetry, Prose, Film.
    ///
    /// - Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub genre: Option<String>,
    /// A description of the intended audience for a piece of evidence.
    ///
    /// - Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub audience: Option<String>,
}

impl super::ToJsonLd for Obj {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants, objects::ToJsonLd};

    const EXP_JSON_LD_SIMPLE: &str = r#"{
  "type": "Evidence",
  "id": "https://raw.githubusercontent.com/hoijui/obadgen/master/res/ob-ents/evidence-1.json"
}"#;

    const EXP_JSON_LD_ALL: &str = r#"{
  "type": "Evidence",
  "id": "https://raw.githubusercontent.com/hoijui/obadgen/master/res/ob-ents/evidence-2.json",
  "narrative": "Much, much longer description.",
  "name": "Evidence 2",
  "description": "Short description",
  "genre": "Poetry",
  "audience": "To whom it may concern"
}"#;

    #[test]
    fn test_builder_simple() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder().id(constants::EVIDENCE_1_ID).build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);
        Ok(())
    }

    #[test]
    fn test_builder_all() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder()
            .id(constants::EVIDENCE_2_ID)
            .name("Evidence 2")
            .description("Short description")
            .narrative("Much, much longer description.")
            .genre("Poetry")
            .audience("To whom it may concern")
            .build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_ALL);
        Ok(())
    }
}

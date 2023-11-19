// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

/// Descriptive metadata about the achievements necessary
/// to be recognized with an `Assertion` of a particular `BadgeClass`.
/// This data is added to the `BadgeClass`
/// so that it may be rendered when that `BadgeClass` is displayed,
/// instead of simply a link to human-readable criteria
/// external to the badge.
/// Embedding criteria allows either enhancement of an external criteria page
/// or increased portability and ease of use
/// by allowing issuers to skip hosting
/// the formerly-required external criteria page altogether.
///
/// `Criteria` is used to allow would-be recipients
/// to learn what is required of them
/// to be recognized with an `Assertion` of a particular `BadgeClass`.
/// It is also used after the `Assertion` is awarded to a recipient
/// to let those inspecting earned badges know
/// the general requirements that the recipients met
/// in order to earn it.
///
/// On the surface, `Criteria` is a very simple class,
/// but it enables some powerful use cases,
/// such as using a Markdown-formatted narrative
/// to draw the connections between multiple elements in an alignment array.
/// The open nature of the Open Badges vocabulary
/// allows experimentation with [extensions](
/// http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/extensions/)
/// in `Criteria` as well,
/// to let the market establish patterns for machine-readable criteria
/// and automatic-awarding badge contracts.
///
/// See the [definition (& example)](
/// http://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#Criteria).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Criteria")]
pub struct Obj {
    /// Defaults to `Criteria`.
    ///
    /// - JSON-LD Type (Multiple values allowed)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default=Some("Criteria".to_string()))]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub r#type: Option<String>,
    /// The URI of a webpage that describes in a human-readable format
    /// the criteria for the `BadgeClass`.
    ///
    /// - IRI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub id: Option<String>,
    /// A narrative of what is needed to earn the badge.
    ///
    /// - Text or Markdown Text
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub narrative: Option<String>,
}

impl super::ToJsonLd for Obj {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{constants, objects::ToJsonLd};

    const EXP_JSON_LD_SIMPLE: &str = r#"{
  "type": "Criteria",
  "id": "https://raw.githubusercontent.com/hoijui/obadgen/master/res/ob-ents/criteria-1.json"
}"#;

    const EXP_JSON_LD_ALL: &str = r#"{
  "type": "Criteria",
  "id": "https://raw.githubusercontent.com/hoijui/obadgen/master/res/ob-ents/criteria-2.json",
  "narrative": "Much, much longer description."
}"#;

    #[test]
    fn test_builder_simple() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder().id(constants::CRITERIA_1_ID).build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);
        Ok(())
    }

    #[test]
    fn test_builder_all() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder()
            .id(constants::CRITERIA_2_ID)
            .narrative("Much, much longer description.")
            .build();
        let json_ld = obj.to_json_ld()?;
        assert_eq!(&json_ld, EXP_JSON_LD_ALL);
        Ok(())
    }
}

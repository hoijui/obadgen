// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use biscuit::CompactJson;
use monostate::MustBe;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::serde_date_time::SerdeDateTime;
use crate::util::defaults::default_bool;

use super::identity::Obj as Identity;
use super::verification::Obj as Verification;
use super::ToJsonLd;

/// Assertions are representations of an awarded badge,
/// used to share information about a badge belonging to one earner.
/// Assertions are packaged for transmission as JSON objects
/// with a set of mandatory and optional properties.
/// Fields marked in bold letters are mandatory.
/// See the [definition (& example)](
/// https://www.imsglobal.org/sites/default/files/Badges/OBv2p0Final/index.html#Assertion).
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[serde(rename = "Assertion")]
pub struct Obj {
    #[serde(rename = "@context")]
    #[builder(default = MustBe!("https://w3id.org/openbadges/v2"))]
    pub context: MustBe!("https://w3id.org/openbadges/v2"),
    /// valid JSON-LD representation of the Assertion type.
    /// In most cases, this will simply be the string Assertion.
    /// An array including Assertion and other string elements
    /// that are either URLs or compact IRIs
    /// within the current context are allowed.
    #[builder(default = MustBe!("Assertion"))]
    pub r#type: MustBe!("Assertion"),
    /// Unique IRI for the Assertion. If using hosted verification,
    /// this should be the URI where the assertion is accessible.
    /// For signed Assertions,
    /// it is recommended to use a UUID in the `urn:uuid`` namespace.
    #[builder(setter(into))]
    pub id: String,
    /// IRI or document that describes the type of badge being awarded.
    /// If an HTTP/HTTPS IRI The endpoint should be a BadgeClass.
    #[builder(setter(into))]
    pub badge: String,
    /// The recipient of the achievement.
    pub recipient: Identity,
    /// Instructions for third parties to verify this assertion.
    /// (Alias "verify" may be used in context.)
    #[serde(alias = "verify")]
    pub verification: Verification,
    /// Timestamp of when the achievement was awarded.
    #[builder(setter(into))]
    pub issued_on: SerdeDateTime,
    /// IRI or document representing an image representing this user’s achievement.
    /// This must be a PNG or SVG image,
    /// and should be prepared via the Baking specification.
    /// An 'unbaked' image for the badge is defined in the BadgeClass
    /// and should not be duplicated here.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub image: Option<String>,
    /// IRI or document describing the work that the recipient did
    /// to earn the achievement.
    /// This can be a page that links out to other pages
    /// if linking directly to the work is infeasible.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    #[builder(default)]
    #[builder(setter(into))]
    pub evidence: Vec<String>,
    /// A narrative that connects multiple pieces of evidence.
    /// Likely only present at this location if evidence is a multi-value array.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub narrative: Option<String>,
    /// If the achievement has some notion of expiry,
    /// this indicates a timestamp when a badge should no longer be considered valid.
    /// After this time, the badge should be considered expired.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub expires: Option<SerdeDateTime>,
    /// Defaults to false if `Assertion` is not referenced
    /// from a `revokedAssertions` list and may be omitted.
    /// See `RevocationList`.
    /// If revoked is true,
    /// only revoked and id are required properties,
    /// and many issuers strip a hosted `Assertion` down
    /// to only those properties when revoked.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    // #[serde(default)] // This is also `false`
    #[serde(default = "default_bool::<false>")]
    #[builder(default = false)]
    pub revoked: bool,
    /// Optional published reason for revocation, if revoked.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    #[builder(setter(strip_option))]
    #[builder(setter(into))]
    pub revocation_reason: Option<String>,
}

impl Obj {
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

impl CompactJson for Obj {}
impl ToJsonLd for Obj {}

#[cfg(test)]
mod tests {
    use super::super::identity::Obj as Identity;
    use super::super::identity::ObjType as IdentityType;
    use super::super::verification::Obj as Verification;
    use super::super::verification::ObjType as VerificationType;
    use super::*;
    use crate::constants;
    use chrono::DateTime;

    const EXP_JSON_LD_SIMPLE: &str = r#"{
  "@context": "https://w3id.org/openbadges/v2",
  "type": "Assertion",
  "id": "https://raw.githubusercontent.com/hoijui/obadgen/master/res/ob-ents/badge-assertion-simple.json",
  "badge": "https://raw.githubusercontent.com/hoijui/obadgen/master/res/ob-ents/badge-definition-simple.json",
  "recipient": {
    "type": "email",
    "identity": "sha256$488842626ec74a0468d90ea17dc4e11c2d0e8e54e45c5075fbd1d2e767f44249",
    "hashed": true
  },
  "verification": {
    "type": "HostedBadge"
  },
  "issuedOn": "2022-06-17T23:59:59Z",
  "expires": "2099-06-30T23:59:59Z"
}"#;

    #[test]
    fn test_new_hosted() -> Result<(), Box<dyn std::error::Error>> {
        let mut obj = Obj::new(
            constants::BADGE_ASSERTION_SIMPLE_ID,
            constants::BADGE_DEFINITION_SIMPLE_ID,
            Identity {
                r#type: IdentityType::EMail,
                hashed: true,
                identity: constants::BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_UNSALTED.clone(),
                salt: None,
            },
            Verification::new(VerificationType::HostedBadge),
            DateTime::parse_from_rfc3339(constants::DT_PAST)?,
        );
        obj.expires = Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());
        let json_ld = obj.to_json_ld()?;

        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);

        Ok(())
    }
    #[test]
    fn test_builder_hosted() -> Result<(), Box<dyn std::error::Error>> {
        let obj = Obj::builder()
            .id(constants::BADGE_ASSERTION_SIMPLE_ID)
            .badge(constants::BADGE_DEFINITION_SIMPLE_ID)
            .recipient(
                Identity::builder()
                    .r#type(IdentityType::EMail)
                    .hashed(true)
                    .identity(constants::BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_UNSALTED.as_str())
                    .build(),
            )
            .verification(
                Verification::builder()
                    .r#type(VerificationType::HostedBadge)
                    .build(),
            )
            .issued_on(DateTime::parse_from_rfc3339(constants::DT_PAST)?)
            .expires(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?)
            .build();
        let json_ld = obj.to_json_ld()?;

        // let simple_json_our_own = simple.serialize_to_json();
        // std::fs::write("simple_json_our_own.json", &simple_json_our_own)?;

        assert_eq!(&json_ld, EXP_JSON_LD_SIMPLE);

        // assert_eq!(simple_json_our_own, simple_json_serde);

        Ok(())
    }
}

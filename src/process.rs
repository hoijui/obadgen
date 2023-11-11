// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::borrow::Cow;

use chrono::DateTime;

use crate::environment::Environment;
use crate::open_badge::{Identity, Verification};
use crate::{box_err::BoxResult, patcher, patcher::Patcher};
use crate::{constants, hash, signature};

/// The main function of this crate,
/// TODO
///
/// # Errors
///
/// TODO
pub fn run(environment: &mut Environment) -> BoxResult<()> {
    let verify_url = "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json";
    let verify_url = if true {
        let use_key = true;
        if use_key {
            let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
            let mut badge_assert = crate::open_badge::BadgeAssertion::new(
                constants::BADGE_ASSERTION_WITH_KEY_ID.to_string(),
                constants::BADGE_DEFINITION_WITH_KEY_ID.to_string(),
                Identity {
                    r#type: crate::open_badge::IdentityType::EMail,
                    hashed: true,
                    identity: email_hash,
                    salt: Some(constants::EMAIL_SALT.to_string()),
                },
                Verification::new(crate::open_badge::VerificationType::SignedBadge {
                    creator: Some(constants::KEY_ID.to_string()),
                }),
                DateTime::parse_from_rfc3339(constants::DT_PAST)?,
            );
            badge_assert.expires =
                Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());
            // let private_key_str = fs::read_to_string(constants::ISSUER_KEY_PATH_PRIV)?;
            let key_priv =
                biscuit::jws::Secret::rsa_keypair_from_file("example.com.priv_pair.der")?;
            let content = signature::sign(badge_assert, &key_priv)?;
            // log::debug!("XXX\n{content}\nXXX");
            // fs::write("badge_assert_plain.txt", &content)?;
            // fs::write("badge_assert_jws.txt", &content)?;
            Cow::Owned(content)
        } else {
            let mut badge_assert = crate::open_badge::BadgeAssertion::new(
                "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json".to_string(),
                "https://thejeshgn.github.io/openbadge/reader-badge.json".to_string(),
                Identity {
                    r#type: crate::open_badge::IdentityType::EMail,
                    hashed: true,
                    identity:
                        "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130"
                            .to_string(),
                    salt: None,
                },
                Verification::new(crate::open_badge::VerificationType::HostedBadge),
                DateTime::parse_from_rfc3339(constants::DT_PAST)?,
            );
            badge_assert.expires =
                Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());
            let badge_assert_ser = serde_json::to_string_pretty(&badge_assert)?;
            Cow::Owned(badge_assert_ser)
        }
    } else {
        Cow::Borrowed(verify_url)
    };
    let fail_if_very_present = true;

    let input_file_path = "res/media/img/test.svg";
    let output_file_path = "target/out.svg";
    patcher::svg::Patcher::rewrite(
        input_file_path,
        output_file_path,
        &verify_url,
        fail_if_very_present,
    )?;

    let input_file_path = "res/media/img/test.png";
    let output_file_path = "target/out.png";
    patcher::png::Patcher::rewrite(
        input_file_path,
        output_file_path,
        &verify_url,
        fail_if_very_present,
    )?;

    log::trace!("Done.");

    Ok(())
}

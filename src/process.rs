// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::borrow::Cow;
use std::fs;

use chrono::DateTime;

use crate::environment::Environment;
use crate::open_badge::Type;
use crate::std_error::Error;
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
            let badge_assert = crate::open_badge::BadgeAssertion {
                id: constants::BADGE_ASSERTION_WITH_KEY_ID,
                badge_id: constants::BADGE_DEFINITION_WITH_KEY_ID,
                recipient_salt: Some(constants::EMAIL_SALT),
                recipient_hashed_email: &email_hash,
                verification_public_key: Some(constants::KEY_ID),
                issued_on: DateTime::parse_from_rfc3339("2022-06-17T23:59:59Z")?.into(),
                expires: DateTime::parse_from_rfc3339("2022-06-17T23:59:59Z")?.into(),
            };
            let private_key_str = fs::read_to_string(constants::ISSUER_KEY_PATH_PRIV)?;
            let key_priv = biscuit::jws::Secret::rsa_keypair_from_file("example.com.priv_pair.der")?;
            let content =
                // signature::sign(&badge_assert, private_key_str.as_bytes()).map_err(|err| {
                //     Error::Message(format!(
                //         "Failed to sign because: {}: '{}'",
                //         err.kind(),
                //         err.message()
                //     ))
                // })?;
                // signature::sign(&badge_assert, private_key_str.as_bytes())?;
                signature::sign(badge_assert.to_deserializable(), &key_priv)?;
            log::debug!("XXX\n{content}\nXXX");
            Cow::Owned(content)
        } else {
            let badge_assert = crate::open_badge::BadgeAssertion {
                id: "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
                recipient_salt: None,
                recipient_hashed_email:
                    "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130",
                badge_id: "https://thejeshgn.github.io/openbadge/reader-badge.json",
                verification_public_key: None,
                issued_on: DateTime::parse_from_rfc3339("2022-06-17T23:59:59Z")?.into(),
                expires: DateTime::parse_from_rfc3339("2022-06-17T23:59:59Z")?.into(),
            };
            Cow::Owned(badge_assert.serialize_to_json())
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

    // cert::test()?;

    log::trace!("Done.");

    Ok(())
}

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{Compact, RegisteredHeader, Secret},
};

use crate::{box_err::BoxResult, open_badge::BadgeAssertion};

/// Signs a badge.
///
/// # Errors
///
/// If computing the Message Authentication Code fails.
///
/// # Panics
///
/// If the biscuit crate does something really wrong internally
/// -> Practically, this can never happen.
pub fn sign(badge_assertion: BadgeAssertion, secret_key: &Secret) -> BoxResult<String> {
    let mut header = RegisteredHeader {
        algorithm: SignatureAlgorithm::RS256,
        media_type: Some("JOSE+JSON".to_string()),
        ..Default::default()
    };

    // See: <https://datatracker.ietf.org/doc/html/rfc7515#section-4.1.9>
    header.media_type = Some("JOSE+JSON".to_string());

    let compact = Compact::new_decoded(header.into(), badge_assertion);
    let encoded = compact.into_encoded(secret_key)?;
    Ok(match encoded {
        Compact::Decoded {
            header: _,
            payload: _,
        } => panic!("This can never happen"),
        Compact::Encoded(parts) => parts.encode(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        box_err::BoxResult,
        constants, hash,
        open_badge::{self, Identity, Verification},
    };
    use chrono::DateTime;

    fn sign_and_verify(
        badge_assertion: BadgeAssertion,
        secret_key: &Secret,
        public_key: &Secret,
    ) -> BoxResult<()> {
        let encoded = sign(badge_assertion, secret_key)?;
        // fs::write("badge_assert_jws.txt", &encoded)?;

        let encoded_parsed: Compact<BadgeAssertion, biscuit::Empty> =
            Compact::new_encoded(&encoded);
        // Decodes and verifies the message
        let _decoded = encoded_parsed.decode(public_key, SignatureAlgorithm::RS256)?;

        Ok(())
    }

    #[test]
    fn test_sign() -> BoxResult<()> {
        let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
        let mut badge_assert = open_badge::BadgeAssertion::new(
            constants::BADGE_ASSERTION_WITH_KEY_ID,
            constants::BADGE_DEFINITION_WITH_KEY_ID,
            Identity {
                r#type: crate::open_badge::IdentityType::EMail,
                hashed: true,
                identity: email_hash,
                salt: Some(constants::EMAIL_SALT.to_string()),
            },
            Verification {
                r#type: crate::open_badge::VerificationType::SignedBadge {
                    creator: Some(constants::KEY_ID.to_string()),
                },
                verification_property: None,
                starts_with: None,
                allowed_origins: None,
            },
            DateTime::parse_from_rfc3339(constants::DT_PAST)?,
        );
        badge_assert.expires = Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());

        // let key_priv = fs::read_to_string(constants::ISSUER_KEY_PATH_PRIV)?;
        // let key_pub = fs::read_to_string(constants::ISSUER_KEY_PATH_PUB)?;
        let key_priv = Secret::rsa_keypair_from_file("example.com.priv_pair.der")?;
        let key_pub = Secret::public_key_from_file("example.com.pub.der")?;

        sign_and_verify(badge_assert, &key_priv, &key_pub)?;

        Ok(())
    }
}

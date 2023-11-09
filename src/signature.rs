// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{Compact, Header, RegisteredHeader, Secret},
};
use chrono::TimeZone;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    box_err::BoxResult,
    open_badge::{BadgeAssertion, Type},
};

/// Signs a badge.
///
/// # Errors
///
/// If computing the Message Authentication Code fails.
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
    use std::fs;

    use super::*;
    use crate::{
        box_err::BoxResult,
        constants, hash,
        open_badge::{self, BadgeRecipient, RecipientType, Verification},
    };
    use chrono::DateTime;
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use monostate::MustBe;
    use serde::de::DeserializeOwned;

    fn sign_and_verify(
        badge_assertion: BadgeAssertion,
        secret_key: &Secret,
        public_key: &Secret,
    ) -> BoxResult<()> {
        let encoded = sign(badge_assertion, secret_key)?;

        // fs::write("badge_assert_jws.txt", &encoded)?;

        let encoded_parsed: Compact<BadgeAssertion, biscuit::Empty> =
            Compact::new_encoded(&encoded);
        // Decode and verify the message.
        // let decoded = decode_verify(encoded.as_bytes(), &HmacVerifier::new(secret_key))?; // HACK -> Wrong! should work with public key! :*/
        let decoded = encoded_parsed.decode(public_key, SignatureAlgorithm::RS256)?;

        // assert!(&decoded.claims.eq(badge_assertion));
        // assert_eq!(
        //     decoded.header.get("alg").and_then(|x| x.as_str()),
        //     Some("HS512")
        // );

        Ok(())
    }

    #[test]
    fn test_sign() -> BoxResult<()> {
        let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
        let badge_asser = open_badge::BadgeAssertion {
            context: MustBe!("https://w3id.org/openbadges/v2"),
            r#type: MustBe!("Assertion"),
            id: constants::BADGE_ASSERTION_WITH_KEY_ID.to_string(),
            badge: constants::BADGE_DEFINITION_WITH_KEY_ID.to_string(),
            recipient: BadgeRecipient::EMail {
                hashed: true,
                identity: email_hash,
                salt: Some(constants::EMAIL_SALT.to_string()),
            },
            verification: Verification::SignedBadge {
                creator: constants::KEY_ID.to_string(),
            },
            issued_on: DateTime::parse_from_rfc3339(constants::DT_PAST)?.into(),
            expires: DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into(),
        };

        // let key_priv = fs::read_to_string(constants::ISSUER_KEY_PATH_PRIV)?;
        // let key_pub = fs::read_to_string(constants::ISSUER_KEY_PATH_PUB)?;
        let key_priv = Secret::rsa_keypair_from_file("example.com.priv_pair.der")?;
        let key_pub = Secret::public_key_from_file("example.com.pub.der")?;

        sign_and_verify(badge_asser, &key_priv, &key_pub)?;
        // .map_err(|err| {
        //     format!(
        //         "Failed to sign because: {}: '{}'",
        //         err.kind(),
        //         err.message()
        //     )
        // })?;

        Ok(())
    }
}

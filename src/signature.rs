// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

use chrono::TimeZone;
use serde::{Serialize, de::DeserializeOwned};
use biscuit::{jwa::SignatureAlgorithm, jws::{Compact, Header, RegisteredHeader, Secret}};

use crate::{box_err::BoxResult, open_badge::{BadgeAssertion, Type}};

/// Signs a badge.
///
/// # Errors
///
/// If computing the Message Authentication Code fails.
pub fn sign<S: AsRef<str> + Display + Serialize + DeserializeOwned>(
    badge_assertion: BadgeAssertion<S>,
    secret_key: &Secret,
) -> BoxResult<String> {
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
        Compact::Decoded { header: _, payload: _ } => panic!("This can never happen"),
        Compact::Encoded(parts) => parts.encode(),
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::{box_err::BoxResult, constants, hash, open_badge};
    use chrono::DateTime;
    use jsonwebtoken::{DecodingKey, Validation, decode};
    use serde::de::DeserializeOwned;

    const DT_PAST: &str = "2022-06-17T23:59:59Z";
    const DT_FAR_FUTURE: &str = "2099-06-30T23:59:59Z";

    fn sign_and_verify<S: AsRef<str> + Display + Serialize + DeserializeOwned + PartialEq>(
        badge_assertion: BadgeAssertion<S>,
        secret_key: &Secret,
        public_key: &Secret,
    ) -> BoxResult<()> {
        let encoded = sign(badge_assertion, secret_key)?;

        let encoded_parsed: Compact<BadgeAssertion<S>, biscuit::Empty> = Compact::new_encoded(&encoded);
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
            id: constants::BADGE_ASSERTION_WITH_KEY_ID.to_string(),
            badge_id: constants::BADGE_DEFINITION_WITH_KEY_ID.to_string(),
            recipient_salt: Some(constants::EMAIL_SALT.to_string()),
            recipient_hashed_email: email_hash,
            verification_public_key: Some(constants::KEY_ID.to_string()),
            issued_on: DateTime::parse_from_rfc3339(DT_PAST)?.into(),
            expires: DateTime::parse_from_rfc3339(DT_FAR_FUTURE)?.into(),
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
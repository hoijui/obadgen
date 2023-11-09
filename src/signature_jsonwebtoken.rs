// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

use chrono::TimeZone;
use jsonwebtoken::{Algorithm, Header, encode};
use serde::Serialize;

use crate::{box_err::BoxResult, open_badge::{BadgeAssertion, Type}};

// fn encode_decode() -> jws::Result<()> {
//     // Add custom header parameters.
//     let mut header = JsonObject::new();
//     header.insert(String::from("typ"), JsonValue::from("text/plain"));

//     // Encode and sign the message.
//     let encoded = encode_sign(header, b"payload", &Hs512Signer::new(b"secretkey"))?;

//     // Decode and verify the message.
//     let decoded = decode_verify(encoded.data().as_bytes(), &HmacVerifier::new(b"secretkey"))?;

//     assert_eq!(decoded.payload, b"payload");
//     assert_eq!(
//         decoded.header.get("typ").and_then(|x| x.as_str()),
//         Some("text/plain")
//     );

//     Ok(())
// }

/// Signs a badge.
///
/// # Errors
///
/// If computing the Message Authentication Code fails.
pub fn sign<S: AsRef<str> + Display + Serialize>(
    badge_assertion: &BadgeAssertion<S>,
    secret_key: &[u8],
) -> BoxResult<String> {

    let secret_key_parsed = jsonwebtoken::EncodingKey::from_rsa_pem(secret_key)?;

    let mut header = Header::new(Algorithm::RS256);

    // See: <https://datatracker.ietf.org/doc/html/rfc7515#section-4.1.9>
    header.typ = Some("JOSE+JSON".to_string());

    Ok(encode(&header, badge_assertion, &secret_key_parsed)?)
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
        badge_assertion: &BadgeAssertion<S>,
        secret_key: &[u8],
        public_key: &[u8],
    ) -> BoxResult<()> {
        let encoded = sign(badge_assertion, secret_key)?;

        // Decode and verify the message.
        // let decoded = decode_verify(encoded.as_bytes(), &HmacVerifier::new(secret_key))?; // HACK -> Wrong! should work with public key! :*/
        let decoded = decode::<BadgeAssertion<S>>(&encoded, &DecodingKey::from_rsa_pem(public_key)?, &Validation::new(Algorithm::RS256))?;

        assert!(&decoded.claims.eq(badge_assertion));
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
        let key_priv = fs::read_to_string("example.com.key")?;
        let key_pub = fs::read_to_string("example.com.pem")?;
        sign_and_verify(&badge_asser, key_priv.as_bytes(), key_pub.as_bytes())?;
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
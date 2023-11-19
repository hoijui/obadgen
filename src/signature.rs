// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{Compact, Header, RegisteredHeader, Secret},
};
use serde::{Deserialize, Serialize};

use crate::{box_err::BoxResult, Assertion};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct HeaderExtensions {
    #[serde(rename = "kty", skip_serializing_if = "Option::is_none")]
    key_type: Option<String>,
    #[serde(rename = "use", skip_serializing_if = "Option::is_none")]
    r#use: Option<String>,
}

/// Signs a badge.
/// If a certificate chain is given, we try to follow [this article](
/// https://software-factotum.medium.com/validating-rsa-signature-for-a-jws-more-about-jwk-and-certificates-e8a3932669f1)
///
/// # Errors
///
/// If computing the Message Authentication Code fails.
///
/// # Panics
///
/// If the biscuit crate does something really wrong internally
/// -> Practically, this can never happen.
pub fn sign_with_cert(
    badge_assertion: Assertion,
    secret_key: &Secret,
    x509_chain: Option<Vec<String>>,
) -> BoxResult<String> {
    let r#use = x509_chain.as_ref().map(|_| "sig".to_string());
    let header = RegisteredHeader {
        algorithm: SignatureAlgorithm::RS256,
        // See: <https://datatracker.ietf.org/doc/html/rfc7515#section-4.1.9>
        media_type: Some("JOSE+JSON".to_string()),
        x509_chain,
        ..Default::default()
    };

    let header_ext = HeaderExtensions {
        key_type: Some("RSA".to_string()),
        r#use,
    };
    let header = Header {
        registered: header,
        private: header_ext,
    };

    let compact = Compact::new_decoded(header, badge_assertion);
    let encoded = compact.into_encoded(secret_key)?;
    Ok(match encoded {
        Compact::Decoded {
            header: _,
            payload: _,
        } => panic!("This can never happen"),
        Compact::Encoded(parts) => parts.encode(),
    })
}

/// Signs a badge without a certificate.
///
/// # Errors
///
/// If computing the Message Authentication Code fails.
///
/// # Panics
///
/// If the biscuit crate does something really wrong internally
/// -> Practically, this can never happen.
pub fn sign(badge_assertion: Assertion, secret_key: &Secret) -> BoxResult<String> {
    sign_with_cert(badge_assertion, secret_key, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Assertion;
    use crate::Identity;
    use crate::IdentityType;
    use crate::Verification;
    use crate::VerificationType;
    use crate::{box_err::BoxResult, constants};
    use chrono::DateTime;

    fn sign_and_verify(
        badge_assertion: Assertion,
        secret_key: &Secret,
        public_key: &Secret,
    ) -> BoxResult<()> {
        let encoded = sign(badge_assertion, secret_key)?;
        // fs::write("badge_assert_jws.txt", &encoded)?;

        let encoded_parsed: Compact<Assertion, biscuit::Empty> = Compact::new_encoded(&encoded);
        // Decodes and verifies the message
        let _decoded = encoded_parsed.decode(public_key, SignatureAlgorithm::RS256)?;

        Ok(())
    }

    #[test]
    fn test_sign() -> BoxResult<()> {
        let mut badge_assert = Assertion::new(
            constants::BADGE_ASSERTION_WITH_KEY_ID,
            constants::BADGE_DEFINITION_WITH_KEY_ID,
            Identity {
                r#type: IdentityType::EMail,
                hashed: true,
                identity: constants::BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_SALTED.clone(),
                salt: Some(constants::BADGE_ASSERTION_RECIPIENT_SALT.to_string()),
            },
            Verification {
                r#type: VerificationType::SignedBadge {
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

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use biscuit::{
    jwa::SignatureAlgorithm,
    jws::{Compact, Header, RegisteredHeader, Secret},
};
use clap::ValueEnum;
// use ring::signature::RsaKeyPair;
use ring::signature::KeyPair;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumVariantNames, IntoStaticStr};

use crate::{box_err::BoxResult, process::Error, Assertion};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Default)]
pub enum AlgorithmType {
    /// No encryption/signature is included for the JWT.
    /// During verification, the signature _MUST BE_ empty or verification  will fail.
    // #[serde(rename = "none")]
    #[default]
    None,
    /// RSASSA-PKCS1-v1_5 using SHA-256
    RSA,
    /// ECDSA using P-256 and SHA-256
    ECDSA,
}

#[derive(
    Debug,
    ValueEnum,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    Eq,
    PartialEq,
    Clone,
    Copy,
    Default,
)]
pub enum Algorithm {
    /// No encryption/signature is included for the JWT.
    /// During verification, the signature _MUST BE_ empty or verification  will fail.
    // #[serde(rename = "none")]
    #[default]
    None,
    /// RSASSA-PKCS1-v1_5 using SHA-256
    RS256,
    /// RSASSA-PKCS1-v1_5 using SHA-384
    RS384,
    /// RSASSA-PKCS1-v1_5 using SHA-512
    RS512,
    /// ECDSA using P-256 and SHA-256
    ES256,
    /// ECDSA using P-384 and SHA-384
    ES384,
    // /// ECDSA using P-521 and SHA-512 --
    // /// This variant is [unsupported](https://github.com/briansmith/ring/issues/268) and will probably never be.
    // ES512,
}

impl Algorithm {
    #[must_use]
    pub const fn to_sig_alg(self) -> SignatureAlgorithm {
        match self {
            Self::None => SignatureAlgorithm::None,
            Self::RS256 => SignatureAlgorithm::RS256,
            Self::RS384 => SignatureAlgorithm::RS384,
            Self::RS512 => SignatureAlgorithm::RS512,
            Self::ES256 => SignatureAlgorithm::ES256,
            Self::ES384 => SignatureAlgorithm::ES384,
        }
    }

    #[must_use]
    pub const fn r#type(self) -> AlgorithmType {
        match self {
            Self::None => AlgorithmType::None,
            Self::RS256 | Self::RS384 | Self::RS512 => AlgorithmType::RSA,
            Self::ES256 | Self::ES384 => AlgorithmType::ECDSA,
        }
    }
}

fn biscuit_to_process_err(err: &biscuit::errors::Error, key_file: &str) -> Error {
    let hint = if let biscuit::errors::Error::KeyRejected(_key_rejected) = &err {
        Some(
            "
Often, keys generated for use in OpenSSL-based software are
encoded in PEM format, which is not supported by *ring*. PEM-encoded
keys can be re-encoded into DER using an OpenSSL command like this:

```sh
# RSA
openssl rsa -in private_key.pem -outform DER -out private_key.der
# ECDSA
openssl ec -in private_key.pem -outform DER -out private_key.der
```
",
        )
    } else {
        None
    };
    Error::InvalidSigningPrivateKey {
        msg: format!(
            "Failed to decode a valid crypto key from '{key_file}': {err:#?}{}",
            hint.unwrap_or_default(),
        ),
    }
}

/// Loads an RSA or ECDSA private key(-pair) file in DER format.
///
/// # Errors
///
/// If loading from file failed; usually because of:
/// - I/O Error
/// - wrong key encoding
/// - wrong key type
/// - unsupported version of key type
/// - corrupt file content
pub fn load_private_key_pair(alg: Algorithm, key_file: impl AsRef<str>) -> BoxResult<Secret> {
    match alg.r#type() {
        AlgorithmType::None => Err("If you try to sign (indicated by suplying a private key), you also have to specify the signing algorithm explicitly (for security reasons)")?,
        AlgorithmType::RSA => Secret::rsa_keypair_from_file(key_file.as_ref())
            .map_err(|err| biscuit_to_process_err(&err, key_file.as_ref()).into()),
        AlgorithmType::ECDSA => {
            Secret::ecdsa_keypair_from_file(alg.to_sig_alg(), key_file.as_ref())
                .map_err(|err| biscuit_to_process_err(&err, key_file.as_ref()).into())
        }
    }
}

fn extract_public_key(alg: Algorithm, key_pair_priv: &Secret) -> BoxResult<Secret> {
    Ok(match alg.r#type() {
        AlgorithmType::None => todo!(),
        AlgorithmType::RSA => {
            if let Secret::RsaKeyPair(key_pair_priv_inner) = key_pair_priv {
                Secret::PublicKey(key_pair_priv_inner.public_key().as_ref().to_owned())
            } else {
                return Err("Given secret is not an RSA private key(-pair)!")?;
            }
        }
        AlgorithmType::ECDSA => {
            if let Secret::EcdsaKeyPair(key_pair_priv_inner) = key_pair_priv {
                Secret::PublicKey(key_pair_priv_inner.public_key().as_ref().to_owned())
            } else {
                return Err("Given secret is not an ECDSA private key(-pair)!")?;
            }
        }
    })
}

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
    alg: Algorithm,
    secret_key: &Secret,
    x509_chain: Option<Vec<String>>,
) -> BoxResult<String> {
    let r#use = x509_chain.as_ref().map(|_| "sig".to_string());
    let header = RegisteredHeader {
        algorithm: alg.to_sig_alg(),
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
pub fn sign(badge_assertion: Assertion, alg: Algorithm, secret_key: &Secret) -> BoxResult<String> {
    sign_with_cert(badge_assertion, alg, secret_key, None)
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
        alg: Algorithm,
        secret_key: &Secret,
        // public_key: &Secret,
    ) -> BoxResult<()> {
        let encoded = sign(badge_assertion, alg, secret_key)?;
        // fs::write("badge_assert_jws.txt", &encoded)?;
        let encoded_parsed: Compact<Assertion, biscuit::Empty> = Compact::new_encoded(&encoded);

        let public_key = extract_public_key(alg, secret_key)?;
        // Decodes and verifies the message
        let _decoded = encoded_parsed.decode(&public_key, alg.to_sig_alg())?;

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
                    creator: Some(constants::ISSUER_KEY_ID.to_string()),
                },
                verification_property: None,
                starts_with: None,
                allowed_origins: None,
            },
            DateTime::parse_from_rfc3339(constants::DT_PAST)?,
        );
        badge_assert.expires = Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());

        let alg = Algorithm::ES256;

        let key_pair_priv = load_private_key_pair(alg, constants::ISSUER_KEY_PATH_PRIV)?;

        sign_and_verify(badge_assert, alg, &key_pair_priv)?;

        Ok(())
    }
}

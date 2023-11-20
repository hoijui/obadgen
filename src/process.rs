// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::borrow::Cow;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use biscuit::jws::Secret;
use chrono::DateTime;
use thiserror::Error;

use crate::environment::Environment;
use crate::patcher::ImageType;
use crate::signature::Algorithm;
use crate::Assertion;
use crate::Identity;
use crate::IdentityType;
use crate::Verification;
use crate::VerificationType;
use crate::{box_err::BoxResult, patcher, patcher::Patcher};
use crate::{constants, signature};

/// This includes any error that may happen during a bakign attempt.
#[derive(Error, Debug)]
pub enum Error {
    #[error("The given combination of settings does not match/line-up into a vlaid configuration: {msg}")]
    InvalidSettings { msg: String },

    #[error("The given assertion is not valid JSON-LD, or not according to the Open Badge 2.0 specification: {msg}")]
    InvalidAssertionInput { msg: String },

    #[error("There was an issue reading or parsing the given private-key: {msg}")]
    InvalidSigningPrivateKey { msg: String },

    #[error("There was an issue reading or parsing the given x.509 certificate: {msg}")]
    InvalidCertificate { msg: String },

    #[error("There was an issue reading or parsing the given badge source image: {msg}")]
    InvalidSourceImage { msg: String },

    #[error("There was an issue signing the assertion content: {msg}")]
    Signing { msg: String },

    #[error("There was an issue baking or writing the final image: {msg}")]
    Output { msg: String },

    /// Very similar to `std::convert::Infallible`.
    #[error("An error that can never happen: {msg}")]
    Impossible { msg: String },

    // // #[error(transparent)]
    // /// Represents all cases of `std::io::Error`.
    // #[error(transparent)]
    // IO(#[from] std::io::Error),
    /// Represents all other cases of `std::error::Error`.
    #[error(transparent)]
    Boxed(#[from] Box<dyn std::error::Error + Send + Sync>),
    // /// An error made up of only a message.
    // #[error("{0}")]
    // Message(String),
}

/// Runs examples.
///
/// # Errors
///
/// If signing the assertion fails.
///
/// If encoding the assertion fails.
pub fn run_examples() -> BoxResult<()> {
    let verify_url = "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json";
    let verify_url = if true {
        let use_key = true;
        if use_key {
            let mut badge_assert = Assertion::new(
                constants::BADGE_ASSERTION_WITH_KEY_ID.to_string(),
                constants::BADGE_DEFINITION_WITH_KEY_ID.to_string(),
                Identity {
                    r#type: IdentityType::EMail,
                    hashed: true,
                    identity: constants::BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_SALTED.clone(),
                    salt: Some(constants::BADGE_ASSERTION_RECIPIENT_SALT.to_string()),
                },
                Verification::new(VerificationType::SignedBadge {
                    creator: Some(constants::ISSUER_KEY_ID.to_string()),
                }),
                DateTime::parse_from_rfc3339(constants::DT_PAST)?,
            );
            badge_assert.expires =
                Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());
            // let private_key_str = fs::read_to_string(constants::ISSUER_KEY_PATH_PRIV)?;
            let key_priv =
                biscuit::jws::Secret::rsa_keypair_from_file(constants::ISSUER_KEY_PATH_PRIV)?;
            let alg = Algorithm::ES256;
            let content = signature::sign(badge_assert, alg, &key_priv)?;
            // log::debug!("XXX\n{content}\nXXX");
            // fs::write("badge_assert_plain.txt", &content)?;
            // fs::write("badge_assert_jws.txt", &content)?;
            Cow::Owned(content)
        } else {
            let mut badge_assert = Assertion::new(
                "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json".to_string(),
                "https://thejeshgn.github.io/openbadge/reader-badge.json".to_string(),
                Identity {
                    r#type: IdentityType::EMail,
                    hashed: true,
                    identity:
                        "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130"
                            .to_string(),
                    salt: None,
                },
                Verification::new(VerificationType::HostedBadge),
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

fn read_assertion(assertion_loc: &Path) -> BoxResult<Assertion> {
    let assertion: Assertion = serde_json::from_reader(File::open(assertion_loc)?)?;

    if let VerificationType::VerificationObject = assertion.verification.r#type {
        return Err(Error::InvalidSettings {
            msg: format!(
                "An Assertions verification.type can not be {:#?}!",
                assertion.verification.r#type
            ),
        }
        .into());
    }

    Ok(assertion)
}

fn read_priv_key_opt(alg: Algorithm, key_loc_opt: Option<&PathBuf>) -> BoxResult<Option<Secret>> {
    Ok(if let Some(key_loc) = key_loc_opt {
        let key_loc_str =
            key_loc
                .as_os_str()
                .to_str()
                .ok_or_else(|| Error::InvalidSigningPrivateKey {
                    // TODO FIXME Could be solved, if we read the file first and give the content to biscuit
                    msg: format!(
                        "Can only read private key from Unicode path; this is not one: '{}'",
                        key_loc.display()
                    ),
                })?;
        Some(signature::load_private_key_pair(alg, key_loc_str)?)
    } else {
        None
    })
}

fn read_cert_chain_opt(cert_loc_opt: Option<&PathBuf>) -> BoxResult<Option<Vec<String>>> {
    Ok(if let Some(cert_loc) = cert_loc_opt {
        let cert_loc_str =
            cert_loc
                .as_os_str()
                .to_str()
                .ok_or_else(|| Error::InvalidCertificate {
                    // TODO FIXME Could be solved, if we read the file first and give the content to biscuit
                    msg: format!(
                        "Can only read x.509certificate from Unicode path; this is not one: '{}'",
                        cert_loc.display()
                    ),
                })?;
        todo!("Certificate chain loading is not yet implemented, because it is probably not possible from a (standardized format) file, but nneeds to be reconstructed though a very complicated process, that browsers know how to do.");
        return Err(Error::InvalidCertificate {
            msg: format!("Certificate chain loading is not yet implemented, becasue it is probably not possible from a (standardized format) file, but nneeds to be reconstructed though a very complicated process, that browsers know how to do. ... from '{cert_loc_str}'",),
        }.into());
    //         Certificate::
    //         // TODO FIXME Support more key types!
    //         let key = biscuit::jws::Secret::rsa_keypair_from_file(key_loc_str).map_err(|err| {
    //             let hint = if let biscuit::errors::Error::KeyRejected(_key_rejected) = &err {
    //                 Some(
    //                     "
    // Often, keys generated for use in OpenSSL-based software are
    // encoded in PEM format, which is not supported by *ring*. PEM-encoded
    // keys that are in `RSAPrivateKey` format can be re-encoded into DER
    // using an OpenSSL command like this:

    // ```sh
    // openssl rsa -in private_key.pem -outform DER -out private_key.der
    // ```
    // ",
    //                 )
    //             } else {
    //                 None
    //             };
    //             Error::InvalidSigningPrivateKey {
    //                 msg: format!(
    //                     "Failed to decode a valid crypto key from '{key_loc_str}': {err:#?}{}",
    //                     hint.unwrap_or_default()
    //                 ),
    //             }
    //         })?;
    // Some(key)
    } else {
        None
    })
}

fn parse_image_types(
    source_image_loc: &Path,
    baked_loc: &Path,
) -> BoxResult<(ImageType, ImageType)> {
    let source_image_type =
        ImageType::try_from(source_image_loc).map_err(|err| Error::InvalidSettings {
            msg: format!("Invalid source image path: {err:#?}!"),
        })?;
    let baked_type = ImageType::try_from(baked_loc).map_err(|err| Error::InvalidSettings {
        msg: format!("Invalid baked image path: {err:#?}!"),
    })?;

    if source_image_type != baked_type {
        return Err(Error::InvalidSettings { msg: format!(
            "Source and baked image types must be equal, but are: {source_image_type:#?} & {baked_type:#?}") }.into());
    }

    Ok((source_image_type, baked_type))
}

fn create_baking_content(
    assertion: Assertion,
    alg: Algorithm,
    key_priv_opt: Option<Secret>,
    x509_chain: Option<Vec<String>>,
) -> BoxResult<Cow<'static, str>> {
    Ok(if false {
        // TODO FIXME We need to support this too!
        let verify_url = "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json";
        Cow::Borrowed(verify_url)
    } else {
        match (&assertion.verification.r#type, key_priv_opt) {
            (VerificationType::VerificationObject, _) => {
                return Err(Error::Impossible {
                    msg: format!(
                        "We already made sure that the Assertions verification.type is not {:#?}!",
                        assertion.verification.r#type
                    ),
                }
                .into());
            }
            (VerificationType::HostedBadge, Some(_key_priv)) => {
                return Err(Error::InvalidSettings { msg: format!(
                "The Assertions verification.type is {:#?}, but a private-key is also supplied; You must change either of these!",
                assertion.verification.r#type) }.into());
            }
            (VerificationType::SignedBadge { creator: _ }, None) => {
                return Err(Error::InvalidSettings { msg: format!(
                "The Assertions verification.type is {:#?}, but a private-key is *not* supplied; You must change either of these!",
                assertion.verification.r#type) }.into());
            }
            (VerificationType::SignedBadge { creator }, Some(key_priv)) => {
                let content = signature::sign_with_cert(assertion, alg, &key_priv, x509_chain)?;
                // log::debug!("XXX\n{content}\nXXX");
                // fs::write("badge_assert_plain.txt", &content)?;
                // fs::write("badge_assert_jws.txt", &content)?;
                Cow::Owned(content)
            }
            (VerificationType::HostedBadge, None) => {
                let badge_assert_ser = serde_json::to_string_pretty(&assertion)?;
                Cow::Owned(badge_assert_ser)
            }
        }
    })
}

/// The main function of this crate,
/// TODO
///
/// # Errors
///
/// If signing the assertion fails.
///
/// If encoding the assertion fails.
pub fn run(environment: &mut Environment) -> BoxResult<()> {
    if let (Some(assertion_loc), Some(source_image_loc), Some(baked_loc)) = (
        environment.settings.assertion_loc.as_ref(),
        environment.settings.source_image_loc.as_ref(),
        environment.settings.baked_loc.as_ref(),
    ) {
        log::info!("Baking Open Badge Assertion from {assertion_loc:#?} into image file {baked_loc:#?} now ...");

        let assertion = read_assertion(assertion_loc)?;

        let sign_alg = environment.settings.sign_alg;

        let key_loc_opt = environment.settings.sign_key_loc.as_ref();
        let key_priv_opt = read_priv_key_opt(sign_alg, key_loc_opt)?;

        let cert_loc_opt = environment.settings.cert_loc.as_ref();
        let x509_chain_opt = read_cert_chain_opt(cert_loc_opt)?;

        let (source_image_type, _baked_type) =
            parse_image_types(source_image_loc.as_path(), baked_loc.as_path())?;

        let baking_content =
            create_baking_content(assertion, sign_alg, key_priv_opt, x509_chain_opt)?;

        let fail_if_veri_present = true;

        // let patcher: Box<dyn patcher::Patcher> = match source_image_type {
        //     ImageType::Svg => Box::new(patcher::svg::Patcher),
        //     ImageType::Png => Box::new(patcher::png::Patcher),
        // };
        match source_image_type {
            ImageType::Svg => {
                patcher::svg::Patcher::rewrite(
                    source_image_loc,
                    baked_loc,
                    &baking_content,
                    fail_if_veri_present,
                )?;
            }
            ImageType::Png => {
                patcher::png::Patcher::rewrite(
                    source_image_loc,
                    baked_loc,
                    &baking_content,
                    fail_if_veri_present,
                )?;
            }
        }

        log::trace!("Done.");

        Ok(())
    } else {
        log::warn!(
            "At least one of '{}' and '{}' are missing, thus just running examples now ...",
            "file-in/assertion-loc",
            "file-out/baked-loc"
        );
        run_examples()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::is_good_error;

    #[test]
    fn normal_types() {
        is_good_error::<Error>();
    }
}

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// You may run this with:
/// cargo run --example res_gen
use chrono::DateTime;
use obadgen::box_err::BoxResult;
use obadgen::constants;
use obadgen::hash;
use obadgen::open_badge::Type;
use rcgen::generate_simple_self_signed;
use rcgen::RcgenError;
use std::fs;
use tracing::Level;

const DT_PAST: &str = "2022-06-17T23:59:59Z";
const DT_FAR_FUTURE: &str = "2099-06-30T23:59:59Z";
// REUSE-IgnoreStart
const REUSE_EXPRS: &str = r#"SPDX-FileCopyrightText: Robin Vobruba <hoijui.quaero@gmail.com>
SPDX-License-Identifier: Unlicense"#;
// REUSE-IgnoreEnd

fn setup_logging() {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(Level::TRACE)
        .init();
}

fn write_to_file<S: AsRef<str>>(file: &str, data: S) -> std::io::Result<()> {
    fs::write(file, data.as_ref())?;
    fs::write(&format!("{file}.license"), REUSE_EXPRS)?;
    Ok(())
}

fn mkdir(dir: &str) -> std::io::Result<()> {
    fs::create_dir_all(dir)?;
    Ok(())
}

fn write_simple() -> BoxResult<()> {
    let issuer = obadgen::open_badge::Issuer {
        id: constants::ISSUER_SIMPLE_ID,
        name: "Issues - Simple",
        url: constants::ISSUER_SIMPLE_URL,
        public_key: None,
    };
    write_to_file(constants::ISSUER_SIMPLE_PATH, issuer.serialize_to_json())?;

    let badge_def = obadgen::open_badge::BadgeDefinition {
        id: constants::BADGE_DEFINITION_SIMPLE_ID,
        name: "Badge - Simple",
        description: "A simple, hosted badge, with a minimal set of properties",
        image_url: "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png", // TODO Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
        criteria: "http://thejeshgn.com/subscribe", // TODO
        tags: ["tagX", "other-tag"].to_vec(),
        alignment: [].to_vec(),
        issuer: constants::ISSUER_SIMPLE_URL, // TODO ... or should it rather be ..._ID?
    };
    write_to_file(
        constants::BADGE_DEFINITION_SIMPLE_PATH,
        badge_def.serialize_to_json(),
    )?;

    let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
    let badge_asser = obadgen::open_badge::BadgeAssertion {
        id: constants::BADGE_ASSERTION_SIMPLE_ID,
        badge_id: constants::BADGE_DEFINITION_SIMPLE_ID,
        recipient_salt: None,
        recipient_hashed_email: &email_hash,
        verification_public_key: None,
        issued_on: DateTime::parse_from_rfc3339(DT_PAST)?.into(),
        expires: DateTime::parse_from_rfc3339(DT_FAR_FUTURE)?.into(),
    };
    write_to_file(
        constants::BADGE_ASSERTION_SIMPLE_PATH,
        badge_asser.serialize_to_json(),
    )?;

    Ok(())
}

fn write_with_key() -> BoxResult<()> {
    let (_issuer_key_priv, issuer_key_pub) = write_key_pair(
        constants::ISSUER_KEY_PATH_PRIV,
        constants::ISSUER_KEY_PATH_PUB,
    )?;

    let crypto_key = obadgen::open_badge::CryptographicKey {
        id: constants::KEY_ID,
        owner_id: constants::ISSUER_WITH_KEY_ID,
        public_key_pem: &issuer_key_pub,
    };
    write_to_file(constants::KEY_PATH, crypto_key.serialize_to_json())?;

    let issuer = obadgen::open_badge::Issuer {
        id: constants::ISSUER_WITH_KEY_ID,
        name: "Issues - with key",
        url: constants::ISSUER_WITH_KEY_URL,
        public_key: Some(constants::KEY_ID),
    };
    write_to_file(constants::ISSUER_WITH_KEY_PATH, issuer.serialize_to_json())?;

    let badge_def = obadgen::open_badge::BadgeDefinition {
        id: constants::BADGE_DEFINITION_WITH_KEY_ID,
        name: "Badge - with key",
        description: "A signed badge",
        image_url: "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png", // TODO Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
        criteria: "http://thejeshgn.com/subscribe", // TODO
        tags: ["tagX", "other-tag"].to_vec(),
        alignment: [].to_vec(),
        issuer: constants::ISSUER_WITH_KEY_URL, // TODO ... or should it rather be ..._ID?
    };
    write_to_file(
        constants::BADGE_DEFINITION_WITH_KEY_PATH,
        badge_def.serialize_to_json(),
    )?;

    let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
    let badge_asser = obadgen::open_badge::BadgeAssertion {
        id: constants::BADGE_ASSERTION_WITH_KEY_ID,
        badge_id: constants::BADGE_DEFINITION_WITH_KEY_ID,
        recipient_salt: Some(constants::EMAIL_SALT),
        recipient_hashed_email: &email_hash,
        verification_public_key: Some(constants::KEY_ID),
        issued_on: DateTime::parse_from_rfc3339(DT_PAST)?.into(),
        expires: DateTime::parse_from_rfc3339(DT_FAR_FUTURE)?.into(),
    };
    write_to_file(
        constants::BADGE_ASSERTION_WITH_KEY_PATH,
        badge_asser.serialize_to_json(),
    )?;

    Ok(())
}

fn main() -> BoxResult<()> {
    setup_logging();

    mkdir(constants::BASE_HOSTING_PATH)?;

    write_simple()?;
    write_with_key()?;

    Ok(())
}

fn generate_key_pair() -> Result<(String, String), RcgenError> {
    let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];

    let cert = generate_simple_self_signed(subject_alt_names)?;
    // The certificate is now valid for localhost
    // and the domain "hello.world.example"
    println!("{}", cert.serialize_pem()?);
    println!("{}", cert.serialize_private_key_pem());
    Ok((cert.serialize_pem()?, cert.serialize_private_key_pem()))
}

fn write_key_pair(
    issuer_key_path_priv: &str,
    issuer_key_path_pub: &str,
) -> BoxResult<(String, String)> {
    let (key_priv, key_pub) = generate_key_pair()?;
    write_to_file(issuer_key_path_priv, &key_priv)?;
    write_to_file(issuer_key_path_pub, &key_pub)?;
    Ok((key_priv, key_pub))
}

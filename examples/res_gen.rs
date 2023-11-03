// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// You may run this with:
/// cargo run --example res_gen
use chrono::DateTime;
use obadgen::constants;
use obadgen::hash;
use obadgen::BoxResult;
use rcgen::RcgenError;
use std::fs;
use tracing::Level;
use rcgen::generate_simple_self_signed;

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
    let issuer = obadgen::open_badge::create_issuer(
        constants::ISSUER_SIMPLE_ID,
        "Issues - Simple",
        constants::ISSUER_SIMPLE_URL,
        None,
    );
    write_to_file(constants::ISSUER_SIMPLE_PATH, issuer)?;

    let badge_def = obadgen::open_badge::create_badge_definition(
        constants::BADGE_DEFINITION_SIMPLE_ID,
        "Badge - Simple",
        "A simple, hosted badge, with a minimal set of properties",
        "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png", // TODO Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
        "http://thejeshgn.com/subscribe", // TODO
        ["tagX", "other-tag"].to_vec(),
        [].to_vec(),
        constants::ISSUER_SIMPLE_URL, // TODO ... or should it rather be ..._ID?
    );
    write_to_file(constants::BADGE_DEFINITION_SIMPLE_PATH, badge_def)?;

    let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
    let badge_asser = obadgen::open_badge::create_badge_assertion(
        constants::BADGE_ASSERTION_SIMPLE_ID,
        constants::BADGE_DEFINITION_SIMPLE_ID,
        None,
        &email_hash,
        None,
        &DateTime::parse_from_rfc3339(DT_PAST)?,
        &DateTime::parse_from_rfc3339(DT_FAR_FUTURE)?,
    );
    write_to_file(constants::BADGE_ASSERTION_SIMPLE_PATH, badge_asser)?;

    Ok(())
}

fn write_with_key() -> BoxResult<()> {
    write_key_pair(
        constants::ISSUER_KEY_PATH_PRIV,
        constants::ISSUER_KEY_PATH_PUB,
    )?;
    write_key_pair(
        constants::VERIFICATION_KEY_PATH_PRIV,
        constants::VERIFICATION_KEY_PATH_PUB,
    )?;

    let issuer = obadgen::open_badge::create_issuer(
        constants::ISSUER_WITH_KEY_ID,
        "Issues - with key",
        constants::ISSUER_WITH_KEY_URL,
        Some(constants::ISSUER_KEY_PATH_PUB),
    );
    write_to_file(constants::ISSUER_WITH_KEY_PATH, issuer)?;

    let badge_def = obadgen::open_badge::create_badge_definition(
        constants::BADGE_DEFINITION_WITH_KEY_ID,
        "Badge - with key",
        "A signed badge",
        "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png", // TODO Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
        "http://thejeshgn.com/subscribe", // TODO
        ["tagX", "other-tag"].to_vec(),
        [].to_vec(),
        constants::ISSUER_WITH_KEY_URL, // TODO ... or should it rather be ..._ID?
    );
    write_to_file(constants::BADGE_DEFINITION_WITH_KEY_PATH, badge_def)?;

    let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
    let badge_asser = obadgen::open_badge::create_badge_assertion(
        constants::BADGE_ASSERTION_WITH_KEY_ID,
        constants::BADGE_DEFINITION_WITH_KEY_ID,
        Some(constants::EMAIL_SALT),
        &email_hash,
        Some(constants::VERIFICATION_KEY_PATH_PUB),
        &DateTime::parse_from_rfc3339(DT_PAST)?,
        &DateTime::parse_from_rfc3339(DT_FAR_FUTURE)?,
    );
    write_to_file(constants::BADGE_ASSERTION_WITH_KEY_PATH, badge_asser)?;

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

fn write_key_pair(issuer_key_path_priv: &str, issuer_key_path_pub: &str) -> BoxResult<()> {
    let (key_priv, key_pub) = generate_key_pair()?;
    write_to_file(issuer_key_path_priv, key_priv)?;
    write_to_file(issuer_key_path_pub, key_pub)?;
    Ok(())
}

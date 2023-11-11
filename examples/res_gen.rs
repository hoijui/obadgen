// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// You may run this with:
/// cargo run --example res_gen
use chrono::DateTime;
use obadgen::box_err::BoxResult;
use obadgen::constants;
use obadgen::hash;
use obadgen::open_badge::Identity;
use obadgen::open_badge::IdentityType;
use obadgen::open_badge::Type;
use obadgen::open_badge::Verification;
use obadgen::open_badge::VerificationType;
use rcgen::generate_simple_self_signed;
use rcgen::RcgenError;
use std::fs;
use tracing::Level;

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
    let mut issuer = obadgen::open_badge::Issuer::new(constants::ISSUER_SIMPLE_ID);
    issuer.name = Some("Issuer - simple".to_string());
    issuer.url = Some(constants::ISSUER_SIMPLE_URL.to_string());
    write_to_file(constants::ISSUER_SIMPLE_PATH, issuer.to_json_ld()?)?;

    let mut badge_def = obadgen::open_badge::BadgeDefinition::new(
        constants::BADGE_DEFINITION_SIMPLE_ID,
        "Badge - Simple",
        "A simple, hosted badge, with a minimal set of properties",
        "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png", // TODO Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
        "http://thejeshgn.com/subscribe", // TODO
        constants::ISSUER_SIMPLE_URL,     // TODO ... or should it rather be ..._ID?
    );
    badge_def.tags = ["tagX".to_string(), "other-tag".to_string()].to_vec();
    write_to_file(
        constants::BADGE_DEFINITION_SIMPLE_PATH,
        badge_def.to_json_ld()?,
    )?;

    let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
    let mut badge_assert = obadgen::open_badge::BadgeAssertion::new(
        constants::BADGE_ASSERTION_SIMPLE_ID.to_string(),
        constants::BADGE_DEFINITION_SIMPLE_ID.to_string(),
        Identity {
            r#type: IdentityType::EMail,
            hashed: true,
            identity: email_hash,
            salt: None,
        },
        Verification::new(VerificationType::HostedBadge),
        DateTime::parse_from_rfc3339(constants::DT_PAST)?,
    );
    badge_assert.expires = Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());
    let badge_assert_ser = serde_json::to_string_pretty(&badge_assert)?;
    write_to_file(constants::BADGE_ASSERTION_SIMPLE_PATH, badge_assert_ser)?;

    Ok(())
}

fn write_with_key() -> BoxResult<()> {
    let (_issuer_key_priv, issuer_key_pub) = write_key_pair(
        constants::ISSUER_KEY_PATH_PRIV,
        constants::ISSUER_KEY_PATH_PUB,
    )?;

    let crypto_key = obadgen::open_badge::CryptographicKey::new(
        constants::KEY_ID,
        constants::ISSUER_WITH_KEY_ID,
        &issuer_key_pub,
    );
    write_to_file(constants::KEY_PATH, crypto_key.to_json_ld()?)?;

    let mut issuer = obadgen::open_badge::Issuer::new(constants::ISSUER_WITH_KEY_ID);
    issuer.name = Some("Issuer - with key".to_string());
    issuer.url = Some(constants::ISSUER_WITH_KEY_URL.to_string());
    issuer.public_key = Some(constants::KEY_ID.to_string());
    write_to_file(constants::ISSUER_WITH_KEY_PATH, issuer.to_json_ld()?)?;

    let mut badge_def = obadgen::open_badge::BadgeDefinition::new(
        constants::BADGE_DEFINITION_WITH_KEY_ID,
        "Badge - with key",
        "A signed badge",
        "https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png", // TODO Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
        "http://thejeshgn.com/subscribe", // TODO
        constants::ISSUER_WITH_KEY_URL,   // TODO ... or should it rather be ..._ID?
    );
    badge_def.tags = ["tagX".to_string(), "other-tag".to_string()].to_vec();
    write_to_file(
        constants::BADGE_DEFINITION_WITH_KEY_PATH,
        badge_def.to_json_ld()?,
    )?;

    let email_hash = hash::sha256(constants::BADGE_ASSERTION_RECIPIENT_EMAIL);
    let mut badge_assert = obadgen::open_badge::BadgeAssertion::new(
        constants::BADGE_ASSERTION_WITH_KEY_ID.to_string(),
        constants::BADGE_DEFINITION_WITH_KEY_ID.to_string(),
        Identity {
            r#type: IdentityType::EMail,
            hashed: true,
            identity: email_hash,
            salt: Some(constants::EMAIL_SALT.to_string()),
        },
        Verification::new(VerificationType::SignedBadge {
            creator: Some(constants::KEY_ID.to_string()),
        }),
        DateTime::parse_from_rfc3339(constants::DT_PAST)?,
    );
    badge_assert.expires = Some(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?.into());
    let badge_assert_ser = serde_json::to_string_pretty(&badge_assert)?;
    write_to_file(constants::BADGE_ASSERTION_WITH_KEY_PATH, badge_assert_ser)?;

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

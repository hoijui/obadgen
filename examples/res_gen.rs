// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// You may run this with:
/// cargo run --example res_gen
use chrono::DateTime;
use obadgen::box_err::BoxResult;
use obadgen::cert_gen;
use obadgen::constants;
use obadgen::Assertion;
use obadgen::BadgeClass;
use obadgen::CryptographicKey;
use obadgen::Identity;
use obadgen::IdentityType;
use obadgen::Issuer;
use obadgen::ToJsonLd;
use obadgen::Verification;
use obadgen::VerificationType;
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
    let issuer = Issuer::builder()
        .id(constants::ISSUER_SIMPLE_ID)
        .name("Issuer - simple")
        .url(constants::ISSUER_SIMPLE_URL)
        .build();
    write_to_file(constants::ISSUER_SIMPLE_PATH, issuer.to_json_ld()?)?;

    let badge_def = BadgeClass::builder()
        .id(constants::BADGE_DEFINITION_SIMPLE_ID)
        .name("Badge - simple")
        .description("A simple, hosted badge, with a minimal set of properties")
        .image(constants::BADGE_DEFINITION_SIMPLE_IMAGE_PATH)
        .criteria("http://thejeshgn.com/subscribe") // TODO
        .issuer(constants::ISSUER_SIMPLE_ID)
        .tags(["tagX".to_string(), "other-tag".to_string()])
        .build();
    write_to_file(
        constants::BADGE_DEFINITION_SIMPLE_PATH,
        badge_def.to_json_ld()?,
    )?;

    let badge_assert = Assertion::builder()
        .id(constants::BADGE_ASSERTION_SIMPLE_ID)
        .badge(constants::BADGE_DEFINITION_SIMPLE_ID)
        .recipient(
            Identity::builder()
                .r#type(IdentityType::EMail)
                .hashed(true)
                .identity(constants::BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_UNSALTED.as_ref() as &str)
                .build(),
        )
        .verification(Verification::new(VerificationType::HostedBadge))
        .issued_on(DateTime::parse_from_rfc3339(constants::DT_PAST)?)
        .expires(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?)
        .build();
    let badge_assert_ser = serde_json::to_string_pretty(&badge_assert)?;
    write_to_file(constants::BADGE_ASSERTION_SIMPLE_PATH, badge_assert_ser)?;

    Ok(())
}

fn write_with_key() -> BoxResult<()> {
    let subject_alt_names: &[_] = &[
        // TODO Change these!
        "hello.world.example".to_string(),
        "localhost".to_string(),
    ];
    // let cert = cert_gen::create_rsa_cert(subject_alt_names)?;
    let certified_key = rcgen::generate_simple_self_signed(subject_alt_names)?;
    let cert_cont = cert_gen::Container {
        certified_key,
        file_base: constants::ISSUER_CERT_PATH_BASE.into(),
    };
    cert_cont.write_files()?;
    cert_cont.write_license_files(REUSE_EXPRS)?;
    // let (_issuer_key_priv, issuer_key_pub) = write_key_pair(
    //     constants::ISSUER_KEY_PATH_PRIV,
    //     constants::ISSUER_KEY_PATH_PUB,
    // )?;

    let crypto_key = CryptographicKey::builder()
        .id(constants::ISSUER_KEY_ID)
        .owner(constants::ISSUER_WITH_KEY_ID)
        .public_key_pem(cert_cont.certified_key.key_pair.public_key_pem())
        .build();
    write_to_file(constants::ISSUER_KEY_PATH, crypto_key.to_json_ld()?)?;

    let issuer = Issuer::builder()
        .id(constants::ISSUER_WITH_KEY_ID)
        .name("Issuer - with key")
        .url(constants::ISSUER_WITH_KEY_URL)
        .public_key(constants::ISSUER_KEY_ID)
        .build();
    write_to_file(constants::ISSUER_WITH_KEY_PATH, issuer.to_json_ld()?)?;

    let badge_def = BadgeClass::builder()
        .id(constants::BADGE_DEFINITION_WITH_KEY_ID)
        .name("Badge - with key")
        .description("A signed badge")
        .image("https://731860.p3cdn2.secureserver.net/blog/wp-content/uploads/2014/07/thejeshgn_icon.png") // TODO Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
        .criteria("http://thejeshgn.com/subscribe") // TODO
        .issuer(constants::ISSUER_WITH_KEY_URL)     // TODO ... or should it rather be ..._ID?
        .tags(["tagX".to_string(), "other-tag".to_string()])
        .build();
    write_to_file(
        constants::BADGE_DEFINITION_WITH_KEY_PATH,
        badge_def.to_json_ld()?,
    )?;

    let badge_assert = Assertion::builder()
        .id(constants::BADGE_ASSERTION_WITH_KEY_ID)
        .badge(constants::BADGE_DEFINITION_WITH_KEY_ID)
        .recipient(Identity {
            r#type: IdentityType::EMail,
            hashed: true,
            identity: constants::BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_SALTED.to_string(),
            salt: Some(constants::BADGE_ASSERTION_RECIPIENT_SALT.to_string()),
        })
        .verification(Verification::new(VerificationType::SignedBadge {
            creator: Some(constants::ISSUER_KEY_ID.to_string()),
        }))
        .issued_on(DateTime::parse_from_rfc3339(constants::DT_PAST)?)
        .expires(DateTime::parse_from_rfc3339(constants::DT_FAR_FUTURE)?)
        .build();
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

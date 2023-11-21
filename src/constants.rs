use const_format::formatcp;

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use lazy_static::lazy_static;

use crate::hash;

/// The default date format.
/// For formatting specifiers, see:
/// <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
pub const DATE_FORMAT_GIT: &str = "%Y-%m-%d %H:%M:%S";

// pub const BASE_HOSTING_URL: &str = "https://hoijui.github.io/obadgen";
pub const BASE_HOSTING_URL: &str = "https://raw.githubusercontent.com/hoijui/obadgen/master";
pub const BASE_HOSTING_PATH: &str = "res/ob-ents";
// pub const BASE_ENTS_URL: &str = const_concat!(BASE_HOSTING_URL, '/', BASE_HOSTING_PATH);
pub const BASE_ENTS_URL: &str = formatcp!("{BASE_HOSTING_URL}/{BASE_HOSTING_PATH}");

// pub const ISSUER_SIMPLE_PATH: &str = const_concat!(BASE_HOSTING_PATH, '/', "issuer-simple.json");
// pub const ISSUER_SIMPLE_URL: &str = const_concat!(BASE_HOSTING_URL, '/', ISSUER_SIMPLE_PATH);
// pub const ISSUER_SIMPLE_ID: &str = ISSUER_SIMPLE_URL;

// pub const BADGE_DEFINITION_SIMPLE_PATH: &str =
//     const_concat!(BASE_HOSTING_PATH, '/', "badge-definition-simple.json");
// pub const BADGE_DEFINITION_SIMPLE_URL: &str =
//     const_concat!(BASE_HOSTING_URL, '/', BADGE_DEFINITION_SIMPLE_PATH);
// pub const BADGE_DEFINITION_SIMPLE_ID: &str = BADGE_DEFINITION_SIMPLE_URL;

// pub const BADGE_ASSERTION_SIMPLE_PATH: &str =
//     const_concat!(BASE_HOSTING_PATH, '/', "badge-assertion-simple.json");
// pub const BADGE_ASSERTION_SIMPLE_URL: &str =
//     const_concat!(BASE_HOSTING_URL, '/', BADGE_ASSERTION_SIMPLE_PATH);
// pub const BADGE_ASSERTION_SIMPLE_ID: &str = BADGE_ASSERTION_SIMPLE_URL;

pub const ISSUER_SIMPLE_PATH: &str = formatcp!("{BASE_HOSTING_PATH}/issuer-simple.json");
pub const ISSUER_SIMPLE_URL: &str = formatcp!("{BASE_HOSTING_URL}/{ISSUER_SIMPLE_PATH}");
pub const ISSUER_SIMPLE_ID: &str = ISSUER_SIMPLE_URL;

pub const BADGE_DEFINITION_SIMPLE_PATH: &str =
    formatcp!("{BASE_HOSTING_PATH}/badge-definition-simple.json");
pub const BADGE_DEFINITION_SIMPLE_URL: &str =
    formatcp!("{BASE_HOSTING_URL}/{BADGE_DEFINITION_SIMPLE_PATH}");
pub const BADGE_DEFINITION_SIMPLE_ID: &str = BADGE_DEFINITION_SIMPLE_URL;

// TODO Actually create this image! - Make our own set of badges for teting, and while we're at it, also for OSH, OSEG & OSEG-OSH!
pub const BADGE_DEFINITION_SIMPLE_IMAGE_PATH: &str =
    formatcp!("{BASE_HOSTING_PATH}/badge-definition-simple-image.png");
pub const BADGE_DEFINITION_SIMPLE_IMAGE_URL: &str =
    formatcp!("{BASE_HOSTING_URL}/{BADGE_DEFINITION_SIMPLE_IMAGE_PATH}");
pub const BADGE_DEFINITION_SIMPLE_IMAGE_ID: &str = BADGE_DEFINITION_SIMPLE_IMAGE_URL;

pub const BADGE_ASSERTION_SIMPLE_PATH: &str =
    formatcp!("{BASE_HOSTING_PATH}/badge-assertion-simple.json");
pub const BADGE_ASSERTION_SIMPLE_URL: &str =
    formatcp!("{BASE_HOSTING_URL}/{BADGE_ASSERTION_SIMPLE_PATH}");
pub const BADGE_ASSERTION_SIMPLE_ID: &str = BADGE_ASSERTION_SIMPLE_URL;
pub const BADGE_ASSERTION_RECIPIENT_EMAIL: &str = "recipient@email.com";
pub const BADGE_ASSERTION_RECIPIENT_SALT: &str = "dfvnk0923#%^&87t6iubasr";
lazy_static! {
    pub static ref BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_UNSALTED: String = hash::sha256(BADGE_ASSERTION_RECIPIENT_EMAIL);
    pub static ref BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_SALTED: String = hash::sha256_with_salt(BADGE_ASSERTION_RECIPIENT_EMAIL, BADGE_ASSERTION_RECIPIENT_SALT);
    // pub static ref BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_UNSALTED: &'static str = &BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_UNSALTED_OWNED;
    // pub static ref BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_SALTED: &'static str = &BADGE_ASSERTION_RECIPIENT_EMAIL_HASH_SALTED_OWNED;
}

pub const ISSUER_CERT_PATH_BASE: &str = formatcp!("{BASE_HOSTING_PATH}/issuer-key");
pub const ISSUER_KEY_PATH: &str = formatcp!("{ISSUER_CERT_PATH_BASE}.json");
pub const ISSUER_KEY_URL: &str = formatcp!("{BASE_HOSTING_URL}/{ISSUER_KEY_PATH}");
pub const ISSUER_KEY_ID: &str = ISSUER_KEY_URL;
pub const ISSUER_KEY_PATH_PRIV: &str = formatcp!("{ISSUER_CERT_PATH_BASE}.priv.der");
pub const ISSUER_CERT_PATH_PUB: &str = formatcp!("{ISSUER_CERT_PATH_BASE}.cert.pem");

pub const ISSUER_WITH_KEY_PATH: &str = formatcp!("{BASE_HOSTING_PATH}/issuer-with-key.json");
pub const ISSUER_WITH_KEY_URL: &str = formatcp!("{BASE_HOSTING_URL}/{ISSUER_WITH_KEY_PATH}");
pub const ISSUER_WITH_KEY_ID: &str = ISSUER_WITH_KEY_URL;

pub const BADGE_DEFINITION_WITH_KEY_PATH: &str =
    formatcp!("{BASE_HOSTING_PATH}/badge-definition-with-key.json");
pub const BADGE_DEFINITION_WITH_KEY_URL: &str =
    formatcp!("{BASE_HOSTING_URL}/{BADGE_DEFINITION_WITH_KEY_PATH}");
pub const BADGE_DEFINITION_WITH_KEY_ID: &str = BADGE_DEFINITION_WITH_KEY_URL;

pub const BADGE_ASSERTION_WITH_KEY_PATH: &str =
    formatcp!("{BASE_HOSTING_PATH}/badge-assertion-with-key.json");
pub const BADGE_ASSERTION_WITH_KEY_URL: &str =
    formatcp!("{BASE_HOSTING_URL}/{BADGE_ASSERTION_WITH_KEY_PATH}");
pub const BADGE_ASSERTION_WITH_KEY_ID: &str = BADGE_ASSERTION_WITH_KEY_URL;

pub const EVIDENCE_1_PATH: &str = formatcp!("{BASE_HOSTING_PATH}/evidence-1.json");
pub const EVIDENCE_1_URL: &str = formatcp!("{BASE_HOSTING_URL}/{EVIDENCE_1_PATH}");
pub const EVIDENCE_1_ID: &str = EVIDENCE_1_URL;

pub const EVIDENCE_2_PATH: &str = formatcp!("{BASE_HOSTING_PATH}/evidence-2.json");
pub const EVIDENCE_2_URL: &str = formatcp!("{BASE_HOSTING_URL}/{EVIDENCE_2_PATH}");
pub const EVIDENCE_2_ID: &str = EVIDENCE_2_URL;

pub const CRITERIA_1_PATH: &str = formatcp!("{BASE_HOSTING_PATH}/criteria-1.json");
pub const CRITERIA_1_URL: &str = formatcp!("{BASE_HOSTING_URL}/{CRITERIA_1_PATH}");
pub const CRITERIA_1_ID: &str = CRITERIA_1_URL;

pub const CRITERIA_2_PATH: &str = formatcp!("{BASE_HOSTING_PATH}/criteria-2.json");
pub const CRITERIA_2_URL: &str = formatcp!("{BASE_HOSTING_URL}/{CRITERIA_2_PATH}");
pub const CRITERIA_2_ID: &str = CRITERIA_2_URL;

pub const DT_PAST: &str = "2022-06-17T23:59:59Z";
pub const DT_FAR_FUTURE: &str = "2099-06-30T23:59:59Z";

use const_format::formatcp;

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

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

pub const BADGE_ASSERTION_SIMPLE_PATH: &str =
    formatcp!("{BASE_HOSTING_PATH}/badge-assertion-simple.json");
pub const BADGE_ASSERTION_SIMPLE_URL: &str =
    formatcp!("{BASE_HOSTING_URL}/{BADGE_ASSERTION_SIMPLE_PATH}");
pub const BADGE_ASSERTION_SIMPLE_ID: &str = BADGE_ASSERTION_SIMPLE_URL;
pub const BADGE_ASSERTION_RECIPIENT_EMAIL: &str = "recipient@email.com";
// pub const BADGE_ASSERTION_SIMPLE_RECIPIENT_EMAIL_HASH: &str =
//     sha256!(BADGE_ASSERTION_RECIPIENT_EMAIL);

pub const KEY_PATH: &str = formatcp!("{BASE_HOSTING_PATH}/key.json");
pub const KEY_URL: &str = formatcp!("{BASE_HOSTING_URL}/{ISSUER_WITH_KEY_PATH}");
pub const KEY_ID: &str = ISSUER_WITH_KEY_URL;

pub const ISSUER_KEY_PATH_PRIV: &str = formatcp!("{BASE_HOSTING_PATH}/issuer-key.priv");
pub const ISSUER_KEY_PATH_PUB: &str = formatcp!("{BASE_HOSTING_PATH}/issuer-key.pub");
pub const EMAIL_SALT: &str = "abcdefg123456789";

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

pub const DT_PAST: &str = "2022-06-17T23:59:59Z";
pub const DT_FAR_FUTURE: &str = "2099-06-30T23:59:59Z";
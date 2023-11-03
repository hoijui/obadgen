// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::environment::Environment;
use crate::{patcher, patcher::Patcher, BoxResult};

/// The main function of this crate,
/// TODO
///
/// # Errors
///
/// TODO
pub fn run(environment: &mut Environment) -> BoxResult<()> {
    // This generates OpenBadge 2.0 compatible JSON-LD that represents an issue of a badge for an individual.
    let badge_issue = r#"{
        "@context": "https://w3id.org/openbadges/v2",
        "type": "Assertion",
        "id": "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json",
        "recipient":
        {
            "type": "email",
            "hashed": true,
            "identity": "sha256$2439c199971e44a07babc5854f5a7fae04028f1c85f492a70bddfa9f55d54130"
        },
        "badge": "https://thejeshgn.github.io/openbadge/reader-badge.json",
        "verification":
        {
            "type": "hosted"
        },
        "issuedOn": "2022-06-17T23:59:59Z",
        "expires": "2030-06-30T23:59:59Z"
    }"#;

    let verify_url = "https://thejeshgn.github.io/openbadge/thejeshgn-reader-badge.json";
    let verify_url = if true { badge_issue } else { verify_url };
    let fail_if_very_present = true;

    let input_file_path = "res/media/img/test.svg";
    let output_file_path = "target/out.svg";
    patcher::svg::Patcher::rewrite(
        input_file_path,
        output_file_path,
        verify_url,
        fail_if_very_present,
    )?;

    let input_file_path = "res/media/img/test.png";
    let output_file_path = "target/out.png";
    patcher::png::Patcher::rewrite(
        input_file_path,
        output_file_path,
        verify_url,
        fail_if_very_present,
    )?;

    // cert::test()?;

    log::trace!("Done.");

    Ok(())
}

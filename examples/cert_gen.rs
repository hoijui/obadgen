// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// You may run this with:
/// cargo run --example cert_gen
use obadgen::box_err::BoxResult;
use rcgen::generate_simple_self_signed;

fn main() -> BoxResult<()> {
    let subject_alt_names = vec!["hello.world.example".to_string(), "localhost".to_string()];

    let certified_key = generate_simple_self_signed(subject_alt_names)?;
    // The certificate is now valid for localhost
    // and the domain "hello.world.example"
    println!("{}", certified_key.cert.pem());
    println!("{}", certified_key.key_pair.serialize_pem());

    Ok(())
}

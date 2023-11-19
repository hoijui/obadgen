// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

// use hex_literal::hex;
use sha2::{Digest, Sha256};

pub fn sha256<S: AsRef<str>>(input: S) -> String {
    let mut hasher = Sha256::new();

    // write input message
    hasher.update(input.as_ref().as_bytes());

    // read hash digest and consume hasher
    let result = hasher.finalize();

    // assert_eq!(result[..], hex!("
    //     b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
    // ")[..]);

    format!("sha256${result:x}")
}

pub fn sha256_with_salt<S1: AsRef<str> + Display, S2: AsRef<str> + Display>(
    input: S1,
    salt: S2,
) -> String {
    sha256(format!("{input}{salt}"))
}

// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use jws::compact::{decode_verify, encode_sign};
use jws::hmac::{HmacVerifier, Hs512Signer};
use jws::{JsonObject, JsonValue};

fn encode_decode() -> jws::Result<()> {
    // Add custom header parameters.
    let mut header = JsonObject::new();
    header.insert(String::from("typ"), JsonValue::from("text/plain"));

    // Encode and sign the message.
    let encoded = encode_sign(header, b"payload", &Hs512Signer::new(b"secretkey"))?;

    // Decode and verify the message.
    let decoded = decode_verify(encoded.data().as_bytes(), &HmacVerifier::new(b"secretkey"))?;

    assert_eq!(decoded.payload, b"payload");
    assert_eq!(
        decoded.header.get("typ").and_then(|x| x.as_str()),
        Some("text/plain")
    );

    Ok(())
}

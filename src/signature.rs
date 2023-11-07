// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fmt::Display;

use chrono::TimeZone;
use jws::compact::{decode_verify, encode_sign};
use jws::hmac::{HmacVerifier, Hs512Signer};
use jws::{JsonObject, JsonValue};

use crate::open_badge::{BadgeAssertion, Type};

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

pub fn sign<S: AsRef<str> + Display, Tz1: TimeZone, Tz2: TimeZone>(
    badge_assertion: &BadgeAssertion<S, Tz1, Tz2>,
    secret_key: &[u8],
) -> jws::Result<String> {
    // Add custom header parameters.
    let mut header = JsonObject::new();
    // header.insert(String::from("typ"), JsonValue::from("text/plain"));
    // header.insert(String::from("alg"), JsonValue::from("RS256"));
    // header.insert(String::from("alg"), JsonValue::from("RS512"));

    let payload = badge_assertion.serialize();
    // Encode and sign the message.
    // let encoded = encode_sign(header, payload.as_bytes(), &Rs256Signer::new(secret_key))?;
    let encoded = encode_sign(header, payload.as_bytes(), &Hs512Signer::new(secret_key))?;

    let verify = true;
    if verify {
        // Decode and verify the message.
        let decoded = decode_verify(encoded.data().as_bytes(), &HmacVerifier::new(secret_key))?;

        assert_eq!(decoded.payload, payload.as_bytes());
        assert_eq!(
            decoded.header.get("alg").and_then(|x| x.as_str()),
            Some("HS512")
        );
    }

    Ok(encoded.into_data())
}

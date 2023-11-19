<!--
SPDX-FileCopyrightText: 2021-2023 Robin Vobruba <hoijui.quaero@gmail.com>

SPDX-License-Identifier: CC0-1.0
-->

# *O*pen-*Bad*ges *Gen*erator (`obadgen`)

[![License: AGPL-3.0-or-later](
    https://img.shields.io/badge/License-AGPL%203.0+-blue.svg)](
    https://www.gnu.org/licenses/agpl-3.0.html)
[![REUSE status](
    https://api.reuse.software/badge/github.com/hoijui/obadgen)](
    https://api.reuse.software/info/github.com/hoijui/obadgen)
<!--
[![crates.io](
    https://img.shields.io/crates/v/obadgen.svg)](
    https://crates.io/crates/obadgen)
-->
[![Docs](
    https://docs.rs/obadgen/badge.svg)](
    https://docs.rs/obadgen)
[![dependency status](
    https://deps.rs/repo/github/hoijui/obadgen/status.svg)](
    https://deps.rs/repo/github/hoijui/obadgen)
[![Build status](
    https://github.com/hoijui/obadgen/workflows/build/badge.svg)](
    https://github.com/hoijui/obadgen/actions)

[![In cooperation with Open Source Ecology Germany](
    https://raw.githubusercontent.com/osegermany/tiny-files/master/res/media/img/badge-oseg.svg)](
    https://opensourceecology.de)

`obadgen` is a CLI ([Command-line Interface](
https://en.wikipedia.org/wiki/Command-line_interface)) tool,
helps in generating basic [OpenBadge](https://openbadges.org/) annotated images.

TODO

Create a self-signed x.509 certificate with RSA keys in PEM format:

```shell
CERT_DOMAIN="example.com"
openssl req \
    -x509 \
    -sha256 \
    -nodes \
    -newkey rsa:4096 \
    -keyform PEM \
    -keyout "$CERT_DOMAIN.x509_cert.priv_key.pem" \
    -days 730 \
    -outform DER \
    -out "$CERT_DOMAIN.x509_cert.cert_incl_pub_key.der"
```

```shell
# Visualize certificate for human eyes
openssl x509 \
    -in "$CERT_DOMAIN.x509_cert.cert_incl_pub_key.der" \
    -inform DER \
    -text
```



Converts an RSA key-pair into the DER format:

```shell
CERT_DOMAIN="example.com"
openssl rsa \
    -RSAPublicKey_in \
    -in "$CERT_DOMAIN.pem" \
    -inform PEM \
    -outform DER \
    -RSAPublicKey_out \
    -out "$CERT_DOMAIN.der"
```

Generates a (DER encoded) RSA key-pair inside a single file:

```shell
CERT_DOMAIN="example.com"
openssl genpkey \
    -algorithm RSA \
    -pkeyopt rsa_keygen_bits:4096 \
    -outform der \
    -out "$CERT_DOMAIN.priv_pair.der"
```

Extracts the public key from the key-pair file
created in the example above:

```shell
CERT_DOMAIN="example.com"
openssl rsa \
    -in "$CERT_DOMAIN.priv_pair.der" \
    -inform DER \
    -RSAPublicKey_out \
    -outform DER \
    -out "$CERT_DOMAIN.pub.der"
```

To converta PEM encoded public key into a DER encoded one:

```shell
CERT_DOMAIN="example.com"
openssl rsa \
    -RSAPublicKey_in \
    -in "$CERT_DOMAIN.pub.pem" \
    -inform PEM \
    -outform DER \
    -RSAPublicKey_out \
    -out "$CERT_DOMAIN.pub.der"
```

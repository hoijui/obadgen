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

```shell
CERT_DOMAIN="example.com"
openssl \
    req \
    -x509 \
    -sha256 \
    -nodes \
    -newkey rsa:4096 \
    -keyout "${CERT_DOMAIN}.key" \
    -days 730 \
    -out "${CERT_DOMAIN}.pem"
```

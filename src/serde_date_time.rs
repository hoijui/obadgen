// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use core::fmt;

use chrono::{DateTime, FixedOffset, SecondsFormat};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SerdeDateTime(DateTime<FixedOffset>);

impl Serialize for SerdeDateTime {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_rfc3339_opts(SecondsFormat::Secs, true))
    }
}

impl From<DateTime<FixedOffset>> for SerdeDateTime {
    fn from(value: DateTime<FixedOffset>) -> Self {
        SerdeDateTime(value)
    }
}

impl TryFrom<&str> for SerdeDateTime {
    type Error = chrono::format::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(DateTime::parse_from_rfc3339(value)?))
    }
}

struct SerdeDateTimeVisitor;

impl<'de> Visitor<'de> for SerdeDateTimeVisitor {
    type Value = SerdeDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an RFC3339 compliant date-time string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(SerdeDateTime(
            DateTime::parse_from_rfc3339(value).map_err(|err| E::custom(err.to_string()))?,
        ))
    }
}

impl<'de> Deserialize<'de> for SerdeDateTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(SerdeDateTimeVisitor)
    }
}

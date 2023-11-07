// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::settings::{Settings, STUB};

pub struct Environment {
    pub settings: Settings,
}

impl Environment {
    #[must_use]
    pub fn new(settings: Settings) -> Self {
        Self { settings }
    }

    #[must_use]
    pub fn stub() -> Self {
        Self::new(STUB.clone())
    }
}

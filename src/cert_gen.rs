// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use rcgen::Certificate;
use rcgen::RcgenError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Represents all cases of `std::io::Error`.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Represents all other cases of `rcgen::RcgenError`.
    #[error(transparent)]
    Rcgen(#[from] RcgenError),
}

pub struct Container {
    pub cert: Certificate,
    pub file_base: PathBuf,
}

impl Container {
    fn file_name_add<P: AsRef<Path>>(base: P, addition: &str) -> PathBuf {
        let mut file_name = base
            .as_ref()
            .file_name()
            .expect("certificate containers file_base has no file name part!")
            .to_os_string();
        file_name.push(addition);
        base.as_ref().with_file_name(file_name)
    }

    fn file_add(&self, addition: &str) -> PathBuf {
        Self::file_name_add(&self.file_base, addition)
    }

    #[must_use]
    pub fn priv_der(&self) -> Vec<u8> {
        self.cert.serialize_private_key_der()
    }

    #[must_use]
    pub fn priv_der_file(&self) -> PathBuf {
        self.file_add(".priv.der")
    }

    #[must_use]
    pub fn priv_pem(&self) -> String {
        self.cert.serialize_private_key_pem()
    }

    #[must_use]
    pub fn priv_pem_file(&self) -> PathBuf {
        self.file_add(".priv.pem")
    }

    /// Returns the public certificate (including the public key), DER encoded.
    ///
    /// # Errors
    ///
    /// If encoding failed.
    pub fn cert_der(&self) -> Result<Vec<u8>, RcgenError> {
        self.cert.serialize_der()
    }

    #[must_use]
    pub fn cert_der_file(&self) -> PathBuf {
        self.file_add(".cert.der")
    }

    /// Returns the public certificate (including the public key), PEM encoded.
    ///
    /// # Errors
    ///
    /// If encoding failed.
    pub fn cert_pem(&self) -> Result<String, RcgenError> {
        self.cert.serialize_pem()
    }

    #[must_use]
    pub fn cert_pem_file(&self) -> PathBuf {
        self.file_add(".cert.pem")
    }

    /// Writes all the versions of the certificates files.
    ///
    /// # Errors
    ///
    /// If encoding failed of the public certificates failed.
    ///
    /// If writing any of the files failed (I/O-Error).
    pub fn write_files(&self) -> Result<(), Error> {
        fs::write(self.priv_der_file(), self.priv_der())?;
        fs::write(self.priv_pem_file(), self.priv_pem())?;
        fs::write(self.cert_der_file(), self.cert_der()?)?;
        fs::write(self.cert_pem_file(), self.cert_pem()?)?;
        Ok(())
    }

    /// Writes the REUSE/deb5 compatible `*.license` files
    /// for all the versions of the certificates files.
    ///
    /// # Errors
    ///
    /// If writing any of the files failed (I/O-Error).
    pub fn write_license_files(&self, content: &str) -> std::io::Result<()> {
        fs::write(
            Self::file_name_add(self.priv_der_file(), ".license"),
            content,
        )?;
        fs::write(
            Self::file_name_add(self.priv_pem_file(), ".license"),
            content,
        )?;
        fs::write(
            Self::file_name_add(self.cert_der_file(), ".license"),
            content,
        )?;
        fs::write(
            Self::file_name_add(self.cert_pem_file(), ".license"),
            content,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test::is_good_error;

    #[test]
    fn normal_types() {
        is_good_error::<Error>();
    }
}

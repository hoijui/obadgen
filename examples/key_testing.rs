// SPDX-FileCopyrightText: 2023 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    env::args,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use k256::pkcs8::DecodePrivateKey;

use obadgen::box_err::{BoxError, BoxResult};

type Tester = (&'static str, for<'a> fn(&'a RawKey) -> BoxResult<()>);
type TestResult = (&'static str, BoxResult<()>);

struct RawKey {
    file_loc: PathBuf,
    file_loc_str: String,
    bytes: Vec<u8>,
    str: Option<String>,
}

macro_rules! str_cont_tester {
    ($content:ident, $tester:expr) => {
        if let Some(content_str) = &$content.str {
            unify($tester(content_str))
        } else {
            Err(
                "Inapplicable because input is in binary format and this method requires text"
                    .into(),
            )
        }
    };
}

fn rsa_rsa_private_key_from_pkcs8_pem(content: &RawKey) -> BoxResult<()> {
    str_cont_tester!(content, rsa::RsaPrivateKey::from_pkcs8_pem)
}

fn rsa_rsa_private_key_from_pkcs8_der(content: &RawKey) -> BoxResult<()> {
    unify(rsa::RsaPrivateKey::from_pkcs8_der(&content.bytes))
}

fn k256_secret_key_from_pkcs8_pem(content: &RawKey) -> BoxResult<()> {
    str_cont_tester!(content, k256::SecretKey::from_pkcs8_pem)
}

fn k256_secret_key_from_pkcs8_der(content: &RawKey) -> BoxResult<()> {
    unify(k256::SecretKey::from_pkcs8_der(&content.bytes))
}

fn k256_secret_key_from_sec1_pem(content: &RawKey) -> BoxResult<()> {
    str_cont_tester!(content, k256::SecretKey::from_sec1_pem)
}

fn k256_secret_key_from_sec1_der(content: &RawKey) -> BoxResult<()> {
    unify(k256::SecretKey::from_sec1_der(&content.bytes))
}

fn k256_secret_key_from_slice(content: &RawKey) -> BoxResult<()> {
    unify(k256::SecretKey::from_slice(&content.bytes))
}

fn p256_secret_key_from_pkcs8_pem(content: &RawKey) -> BoxResult<()> {
    str_cont_tester!(content, p256::SecretKey::from_pkcs8_pem)
}

fn p256_secret_key_from_pkcs8_der(content: &RawKey) -> BoxResult<()> {
    unify(p256::SecretKey::from_pkcs8_der(&content.bytes))
}

fn p256_secret_key_from_sec1_pem(content: &RawKey) -> BoxResult<()> {
    str_cont_tester!(content, p256::SecretKey::from_sec1_pem)
}

fn p256_secret_key_from_sec1_der(content: &RawKey) -> BoxResult<()> {
    unify(p256::SecretKey::from_sec1_der(&content.bytes))
}

fn p256_secret_key_from_slice(content: &RawKey) -> BoxResult<()> {
    unify(p256::SecretKey::from_slice(&content.bytes))
}

fn p384_secret_key_from_pkcs8_pem(content: &RawKey) -> BoxResult<()> {
    str_cont_tester!(content, p384::SecretKey::from_pkcs8_pem)
}

fn p384_secret_key_from_pkcs8_der(content: &RawKey) -> BoxResult<()> {
    unify(p384::SecretKey::from_pkcs8_der(&content.bytes))
}

fn p384_secret_key_from_sec1_pem(content: &RawKey) -> BoxResult<()> {
    str_cont_tester!(content, p384::SecretKey::from_sec1_pem)
}

fn p384_secret_key_from_sec1_der(content: &RawKey) -> BoxResult<()> {
    unify(p384::SecretKey::from_sec1_der(&content.bytes))
}

fn p384_secret_key_from_slice(content: &RawKey) -> BoxResult<()> {
    unify(p384::SecretKey::from_slice(&content.bytes))
}

fn biscuit_jws_secret_rsa_keypair_from_file(content: &RawKey) -> BoxResult<()> {
    unify(biscuit::jws::Secret::rsa_keypair_from_file(
        &content.file_loc_str,
    ))
}

fn biscuit_jws_secret_ecdsa_keypair_from_file_es256(content: &RawKey) -> BoxResult<()> {
    unify(biscuit::jws::Secret::ecdsa_keypair_from_file(
        biscuit::jwa::SignatureAlgorithm::ES256,
        &content.file_loc_str,
    ))
}

fn biscuit_jws_secret_ecdsa_keypair_from_file_es384(content: &RawKey) -> BoxResult<()> {
    unify(biscuit::jws::Secret::ecdsa_keypair_from_file(
        biscuit::jwa::SignatureAlgorithm::ES384,
        &content.file_loc_str,
    ))
}

macro_rules! ntup {
    ($fun:ident) => {
        (stringify!($fun), $fun)
    };
}

const KEY_TESTERS: [Tester; 20] = [
    ntup!(rsa_rsa_private_key_from_pkcs8_pem),
    ntup!(rsa_rsa_private_key_from_pkcs8_der),
    ntup!(k256_secret_key_from_pkcs8_pem),
    ntup!(k256_secret_key_from_pkcs8_der),
    ntup!(k256_secret_key_from_sec1_pem),
    ntup!(k256_secret_key_from_sec1_der),
    ntup!(k256_secret_key_from_slice),
    ntup!(p256_secret_key_from_pkcs8_pem),
    ntup!(p256_secret_key_from_pkcs8_der),
    ntup!(p256_secret_key_from_sec1_pem),
    ntup!(p256_secret_key_from_sec1_der),
    ntup!(p256_secret_key_from_slice),
    ntup!(p384_secret_key_from_pkcs8_pem),
    ntup!(p384_secret_key_from_pkcs8_der),
    ntup!(p384_secret_key_from_sec1_pem),
    ntup!(p384_secret_key_from_sec1_der),
    ntup!(p384_secret_key_from_slice),
    ntup!(biscuit_jws_secret_rsa_keypair_from_file),
    ntup!(biscuit_jws_secret_ecdsa_keypair_from_file_es256),
    ntup!(biscuit_jws_secret_ecdsa_keypair_from_file_es384),
];

fn unify<V>(res: Result<V, impl Into<BoxError>>) -> BoxResult<()> {
    res.map(|_val| ()).map_err(|err| err.into())
}

pub fn test_all_loaders(key_file: impl AsRef<Path>) -> BoxResult<Vec<TestResult>> {
    let bytes = fs::read(key_file.as_ref())?;
    let str = String::from_utf8(bytes.clone()).ok();
    let content = RawKey {
        file_loc: key_file.as_ref().to_path_buf(),
        file_loc_str: key_file
            .as_ref()
            .as_os_str()
            .to_str()
            .expect("Private-Key file path is not UTF-8 compatible")
            .to_owned(),
        bytes,
        str,
    };

    let mut results = Vec::new();
    results.reserve(KEY_TESTERS.len());
    for (test_name, test_fn) in KEY_TESTERS {
        let res = test_fn(&content);
        let res_summary = if res.is_ok() { "Ok" } else { "Failed" };
        log::debug!("Tested key loading method '{test_name}',\n\tresult: {res_summary}");
        results.push((test_name, res));
    }

    Ok(results)
}

pub fn test_all_keys_in(keys_dir: impl AsRef<Path>) -> BoxResult<Vec<(String, Vec<TestResult>)>> {
    let entries = fs::read_dir(keys_dir.as_ref())?;

    let mut results = Vec::new();
    for entry_res in entries {
        let entry = entry_res?;
        let key_file = entry.path();
        log::debug!("Testing key file: {key_file:#?} ...");
        let file_res = test_all_loaders(key_file)?;
        results.push((entry.file_name().to_string_lossy().into_owned(), file_res));
    }

    Ok(results)
}

pub fn all_keys_res_to_md_table(
    mut keys_results: Vec<(String, Vec<TestResult>)>,
    out_file: impl AsRef<Path>,
) -> BoxResult<()> {
    let mut out_file_writer = File::create(out_file.as_ref())?;

    keys_results.sort_by(|a, b| b.0.cmp(&a.0));

    let titles: Vec<_> = keys_results
        .first()
        .expect("Not a single file was tested as key")
        .1
        .iter()
        .map(|(name, _tester)| name)
        .collect();
    out_file_writer.write_all(b"| KEY |")?;
    for title in &titles {
        out_file_writer.write_all(b" ")?;
        out_file_writer.write_all(title.as_bytes())?;
        out_file_writer.write_all(b" |")?;
    }
    out_file_writer.write_all(b" #-OKs |")?;

    out_file_writer.write_all(b"\n|")?;
    for _i in 0..(&titles.len() + 2) {
        out_file_writer.write_all(b" --- |")?;
    }

    let mut num_oks_per_test: Vec<usize> = vec![0; titles.len()];
    for (key_name, key_res) in &keys_results {
        let num_oks = key_res
            .iter()
            .map(|(_name, res)| if res.is_ok() { 1 } else { 0 })
            .fold(0, |acc, val| acc + val);
        if num_oks == 0 {
            continue;
        }
        out_file_writer.write_all(b"\n| ")?;
        out_file_writer.write_all(key_name.as_bytes())?;
        out_file_writer.write_all(b" |")?;
        for (i, (_name, res)) in key_res.iter().enumerate() {
            let res_summary = if res.is_ok() {
                num_oks_per_test[i] += 1;
                "[x]"
            } else {
                "[ ]"
            };
            out_file_writer.write_all(b" ")?;
            out_file_writer.write_all(res_summary.as_bytes())?;
            out_file_writer.write_all(b" |")?;
        }
        out_file_writer.write_all(b" ")?;
        out_file_writer.write_all(num_oks.to_string().as_bytes())?;
        out_file_writer.write_all(b" |")?;
    }

    out_file_writer.write_all(b"\n| #-OKs |")?;
    let mut total_oks = 0;
    for oks in num_oks_per_test {
        total_oks += oks;
        out_file_writer.write_all(b" ")?;
        out_file_writer.write_all(oks.to_string().as_bytes())?;
        out_file_writer.write_all(b" |")?;
    }
    out_file_writer.write_all(b" ")?;
    out_file_writer.write_all(total_oks.to_string().as_bytes())?;
    out_file_writer.write_all(b" |")?;

    log::info!("Writing {:#?} done.", out_file.as_ref());

    Ok(())
}

fn main() -> BoxResult<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    log::info!(
        "Key-load-tester expects two (positional) arguments:
- path to a directory containing private key files of different types and encodings
- path to a results file to be written (Markdown)"
    );
    let args: Vec<_> = args().collect();
    let priv_keys_dir = args
        .get(1)
        .expect("First argument has to be a path to a dir containing private-key files");
    let res_md_file = args
        .get(2)
        .expect("Please supply the path to a (to be created) MD output file as second argument");

    log::info!(
        "Using settings:
- priv_keys_dir: {priv_keys_dir:#?}
- res_md_file: {res_md_file:#?}"
    );

    let all_key_results = test_all_keys_in(priv_keys_dir)?;
    all_keys_res_to_md_table(all_key_results, res_md_file)?;
    Ok(())
}

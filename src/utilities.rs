// SPDX-FileCopyrightText: 2020 HH Partners
//
// SPDX-License-Identifier: MIT

use sha2::{Digest, Sha256};
use spdx_rs::models::SPDX;
use std::{
    fs::{read_to_string, write, File},
    io,
    path::Path,
};

pub fn hash256_for_path<P: AsRef<Path>>(path: P) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    let hash: sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize> =
        sha256.finalize();

    Ok(hex::encode_upper(hash))
}

/// Deserialize [`SPDX`] from a file path. Accepts JSON and YAML.
pub fn deserialize_spdx<P: AsRef<Path>>(path_to_spdx: P) -> anyhow::Result<SPDX> {
    let file_contents = read_to_string(&path_to_spdx)?;
    match path_to_spdx.as_ref().extension() {
        Some(extension) => match extension.to_str() {
            Some("json") => Ok(serde_json::from_str::<SPDX>(&file_contents)?),
            Some("yml") | Some("yaml") => Ok(serde_yaml::from_str::<SPDX>(&file_contents)?),
            _ => Err(anyhow::anyhow!("invalid file extension")),
        },
        None => Err(anyhow::anyhow!("invalid file extension")),
    }
}

/// Serialize [`SPDX`] to a file path.
pub fn serialize_spdx<P: AsRef<Path>>(output_path: P, spdx: &SPDX) -> anyhow::Result<()> {
    let json_string = serde_json::to_string_pretty(&spdx)?;
    write(&output_path, json_string)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_for_path() {
        let actual_hash = hash256_for_path("LICENSE").unwrap();
        let expected_hash = String::from("E53CC20D66C471D974F1264DB690B2ED2660816DA7624E75DF744D0D77BA3728");
        assert_eq!(actual_hash, expected_hash);
    }
}

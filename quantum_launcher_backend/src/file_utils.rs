use std::{fs, path::PathBuf};

use reqwest::Client;

use crate::error::{LauncherError, LauncherResult};

pub fn get_launcher_dir() -> LauncherResult<PathBuf> {
    let config_directory = match dirs::config_dir() {
        Some(d) => d,
        None => return Err(LauncherError::ConfigDirNotFound),
    };
    let launcher_directory = config_directory.join("QuantumLauncher");
    create_dir_if_not_exists(&launcher_directory)
        .map_err(|err| LauncherError::IoError(err, launcher_directory.clone()))?;

    Ok(launcher_directory)
}

pub fn create_dir_if_not_exists(path: &PathBuf) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
    } else {
        Ok(())
    }
}

pub async fn download_file_to_string(client: &Client, url: &str) -> LauncherResult<String> {
    let response = client.get(url).send().await?;
    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(LauncherError::ReqwestStatusError(
            response.status(),
            response.url().clone(),
        ))
    }
}

pub async fn download_file_to_bytes(client: &Client, url: &str) -> LauncherResult<Vec<u8>> {
    let response = client.get(url).send().await?;
    if response.status().is_success() {
        Ok(response.bytes().await?.to_vec())
    } else {
        Err(LauncherError::ReqwestStatusError(
            response.status(),
            response.url().clone(),
        ))
    }
}

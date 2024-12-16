use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use reqwest::Client;

use crate::{error::IoError, io_err};

/// Returns the path to the QuantumLauncher root folder.
pub fn get_launcher_dir() -> Result<PathBuf, IoError> {
    let config_directory = dirs::config_dir().ok_or(IoError::ConfigDirNotFound)?;
    let launcher_directory = config_directory.join("QuantumLauncher");
    std::fs::create_dir_all(&launcher_directory).map_err(|err| IoError::Io {
        error: err,
        path: launcher_directory.clone(),
    })?;

    Ok(launcher_directory)
}

pub async fn download_file_to_string(
    client: &Client,
    url: &str,
    user_agent: bool,
) -> Result<String, RequestError> {
    let mut get = client.get(url);
    if user_agent {
        get = get.header(
            "User-Agent",
            "Mrmayman/quantumlauncher (quantumlauncher.github.io)",
        );
    }
    let response = get.send().await?;
    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(RequestError::DownloadError {
            code: response.status(),
            url: response.url().clone(),
        })
    }
}

pub async fn download_file_to_bytes(
    client: &Client,
    url: &str,
    user_agent: bool,
) -> Result<Vec<u8>, RequestError> {
    let mut get = client.get(url);
    if user_agent {
        get = get.header("User-Agent", "quantumlauncher");
    }
    let response = get.send().await?;
    if response.status().is_success() {
        Ok(response.bytes().await?.to_vec())
    } else {
        Err(RequestError::DownloadError {
            code: response.status(),
            url: response.url().clone(),
        })
    }
}

#[derive(Debug)]
pub enum RequestError {
    DownloadError {
        code: reqwest::StatusCode,
        url: reqwest::Url,
    },
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for RequestError {
    fn from(value: reqwest::Error) -> Self {
        Self::ReqwestError(value)
    }
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::DownloadError { code, url } => write!(
                f,
                "could not send request: download error with code {code}, url {url}"
            ),
            RequestError::ReqwestError(err) => {
                write!(f, "could not send request: reqwest library error: {err}")
            }
        }
    }
}

#[cfg(target_family = "unix")]
pub fn set_executable(path: &Path) -> Result<(), IoError> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path)
        .map_err(io_err!(path))?
        .permissions();
    perms.set_mode(0o755); // rwxr-xr-x
    std::fs::set_permissions(path, perms).map_err(io_err!(path))
}

// #[cfg(unix)]
// use std::os::unix::fs::symlink;

// #[cfg(windows)]
// use std::os::windows::fs::{symlink_dir, symlink_file};

// pub fn create_symlink(src: &Path, dest: &Path) -> Result<(), IoError> {
//     #[cfg(unix)]
//     {
//         symlink(src, dest).map_err(io_err!(src.clone()))
//     }

//     #[cfg(windows)]
//     {
//         if src.is_dir() {
//             symlink_dir(src, dest).map_err(io_err!(src.clone()))
//         } else {
//             symlink_file(src, dest).map_err(io_err!(src.clone()))
//         }
//     }
// }

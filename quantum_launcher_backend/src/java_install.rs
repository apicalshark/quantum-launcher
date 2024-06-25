use std::{error::Error, fmt::Display};

use crate::{
    error::IoError,
    file_utils::{self, RequestError},
    io_err,
    json_structs::{
        json_java_files::{JavaFile, JavaFilesJson},
        json_java_list::{JavaListJson, JavaVersion},
        JsonDownloadError,
    },
};

pub async fn install_java(version: JavaVersion) -> Result<(), JavaInstallError> {
    println!("[info] Started installing {}", version.to_string());
    let java_list_json = JavaListJson::download().await?;
    let java_files_url = java_list_json
        .get_url(version)
        .ok_or(JavaInstallError::NoUrlForJavaFiles)?;

    let client = reqwest::Client::new();
    let json = file_utils::download_file_to_string(&client, &java_files_url).await?;
    let json: JavaFilesJson = serde_json::from_str(&json)?;

    let launcher_dir = file_utils::get_launcher_dir()?;

    let java_installs_dir = launcher_dir.join("java_installs");
    std::fs::create_dir_all(&java_installs_dir).map_err(io_err!(java_installs_dir.to_owned()))?;

    let java_install_dir = java_installs_dir.join(version.to_string());
    std::fs::create_dir_all(&java_install_dir).map_err(io_err!(java_installs_dir.to_owned()))?;

    for (file_name, file) in json.files.iter() {
        println!("[info] Installing file: {file_name}");
        let file_path = java_install_dir.join(file_name);
        match file {
            JavaFile::file {
                downloads,
                executable,
            } => {
                let file_bytes =
                    file_utils::download_file_to_bytes(&client, &downloads.raw.url).await?;
                std::fs::write(&file_path, &file_bytes).map_err(io_err!(file_path.to_owned()))?;
                if *executable {
                    file_utils::set_executable(&file_path)?;
                }
            }
            JavaFile::directory {} => {
                std::fs::create_dir_all(&file_path).map_err(io_err!(file_path))?;
            }
            JavaFile::link { target } => {
                println!("[fixme:install_java] Deal with symlink {file_name} -> {target}")
            }
        }
    }

    println!("[info] Finished installing {}", version.to_string());
    Ok(())
}

#[derive(Debug)]
pub enum JavaInstallError {
    JsonDownload(JsonDownloadError),
    Request(RequestError),
    NoUrlForJavaFiles,
    Serde(serde_json::Error),
    Io(IoError),
}

impl From<JsonDownloadError> for JavaInstallError {
    fn from(value: JsonDownloadError) -> Self {
        Self::JsonDownload(value)
    }
}

impl From<RequestError> for JavaInstallError {
    fn from(value: RequestError) -> Self {
        Self::Request(value)
    }
}

impl From<serde_json::Error> for JavaInstallError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<IoError> for JavaInstallError {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

impl Display for JavaInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JavaInstallError::JsonDownload(err) => write!(f, "{err}"),
            JavaInstallError::NoUrlForJavaFiles => write!(f, "could not find url to download java"),
            JavaInstallError::Request(err) => write!(f, "{err}"),
            JavaInstallError::Serde(err) => write!(f, "{err}"),
            JavaInstallError::Io(err) => write!(f, "{err}"),
        }
    }
}

impl Error for JavaInstallError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn java_install() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(install_java(JavaVersion::Java16)).unwrap();
    }
}

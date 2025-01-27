use std::{fmt::Display, path::PathBuf};

use ql_core::{json::VersionDetails, DownloadError, IoError, JavaInstallError, JsonFileError};

pub enum GameLaunchError {
    Io(IoError),
    DownloadError(DownloadError),
    UsernameIsInvalid(String),
    JsonFile(JsonFileError),
    InstanceNotFound,
    Semver(semver::Error),
    VersionJsonNoArgumentsField(Box<VersionDetails>),
    PathBufToString(PathBuf),
    JavaInstall(JavaInstallError),
    CommandError(std::io::Error),
    ForgeInstallUpgradeTransformPathError,
    ForgeInstallUpgradeStripPrefixError,
}

const FORGE_UPGRADE_MESSAGE: &str = r"outdated forge install. Please uninstall and reinstall.
Select your instance, go to Mods -> Uninstall Forge, then Install Forge.";

impl Display for GameLaunchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error launching game: ")?;
        match self {
            Self::Io(err) => write!(f, "(io) {err}"),
            Self::DownloadError(err) => write!(f, "(download) {err}"),
            Self::UsernameIsInvalid(err) => {
                if err.is_empty() {
                    write!(f, "username is empty")
                } else {
                    write!(f, "username contains spaces: {err}")
                }
            }
            Self::JsonFile(err) => write!(f, "(json file) {err}"),
            Self::InstanceNotFound => write!(f, "instance not found"),
            Self::Semver(err) => write!(f, "(semver) {err}"),
            Self::VersionJsonNoArgumentsField(_) => {
                write!(f, "version json has no arguments field")
            }
            Self::PathBufToString(err) => write!(f, "couldn't convert pathbuf to string: {err:?}"),
            Self::JavaInstall(err) => write!(f, "(java install) {err}"),
            Self::CommandError(err) => write!(f, "(command) {err}"),
            Self::ForgeInstallUpgradeTransformPathError => write!(
                f,
                "error upgrading forge install (transforming path)\n{FORGE_UPGRADE_MESSAGE}"
            ),
            Self::ForgeInstallUpgradeStripPrefixError => write!(
                f,
                "error upgrading forge install (removing prefix)\n{FORGE_UPGRADE_MESSAGE}"
            ),
        }
    }
}

impl From<IoError> for GameLaunchError {
    fn from(err: IoError) -> Self {
        GameLaunchError::Io(err)
    }
}

impl From<JsonFileError> for GameLaunchError {
    fn from(err: JsonFileError) -> Self {
        GameLaunchError::JsonFile(err)
    }
}

impl From<semver::Error> for GameLaunchError {
    fn from(err: semver::Error) -> Self {
        GameLaunchError::Semver(err)
    }
}

impl From<DownloadError> for GameLaunchError {
    fn from(err: DownloadError) -> Self {
        GameLaunchError::DownloadError(err)
    }
}

impl From<JavaInstallError> for GameLaunchError {
    fn from(err: JavaInstallError) -> Self {
        GameLaunchError::JavaInstall(err)
    }
}

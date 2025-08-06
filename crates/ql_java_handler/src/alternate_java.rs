//! A module to install Java from various third party sources
//! (like Amazon Corretto) if Mojang doesn't provide Java for your specific platform.
//!
//! # Here is a table representing java platform support.
//!
//! - In this, any entry with numbers filled in represents "official"
//!   mojang support, i.e. they provide the java install files
//!   for these platforms.
//!
//! - Any entry with __ represents a version not supported by
//!   mojang, but supported through *Amazon Corretto Java*
//!   which we provide an alternate installer for.
//!
//! - Any entry with -- represents a version not supported by
//!   mojang, but installed from
//!   <https://github.com/hmsjy2017/get-jdk>
//!
//! - Any entry with !! represents a version not supported at all.
//!
//! ```txt
//! Linux   x64 :  8 16 17 21
//! Linux   x32 :  8 !! !! !!  <- only java 8; MC 1.16.5 and below
//! Linux   a64 : __ __ __ __  <- corretto
//! Linux   a32 : -- !! !! !!  <- github
//!
//! macOS   x64 :  8 16 17 21
//! macOS   a64 : __ __ 17 21  <- corretto
//!
//! Windows x64 :  8 16 17 21
//! Windows x32 :  8 16 17 __  <- corretto
//! Windows a64 : !! !! 17 21  <- only java 17+; mostly fine,
//!                             but some things like ModLoader might break
//! -------------------
//! x64   means x86_64 (64 bit)
//! x32   means x86    (32 bit)
//! a64   means aarch64 or ARM (64 bit)
//! -------------------
//! ```
//!
//! WTF: So... yeah, enjoy this mess

use std::{io::Cursor, path::Path, sync::mpsc::Sender};

use ql_core::{err, file_utils, GenericProgress};

use crate::{extract_tar_gz, send_progress, JavaInstallError, JavaVersion};

pub async fn install(
    version: JavaVersion,
    java_install_progress_sender: Option<&Sender<GenericProgress>>,
    install_dir: &Path,
) -> Result<(), JavaInstallError> {
    let url = version.get_alternate_url();

    let Some(url) = url else {
        return Err(error_unsupported(version));
    };

    send_progress(
        java_install_progress_sender,
        GenericProgress {
            done: 0,
            total: 2,
            message: Some("Getting compressed archive".to_owned()),
            has_finished: false,
        },
    );
    let file_bytes = file_utils::download_file_to_bytes(url, false).await?;
    send_progress(
        java_install_progress_sender,
        GenericProgress {
            done: 1,
            total: 2,
            message: Some("Extracting archive".to_owned()),
            has_finished: false,
        },
    );
    if url.ends_with("tar.gz") {
        extract_tar_gz(&file_bytes, install_dir).map_err(JavaInstallError::TarGzExtract)?;
    } else if url.ends_with("zip") {
        zip_extract::extract(Cursor::new(&file_bytes), install_dir, true)?;
    } else {
        return Err(JavaInstallError::UnknownExtension(url.to_owned()));
    }
    Ok(())
}

fn error_unsupported(version: JavaVersion) -> JavaInstallError {
    if let JavaVersion::Java16 | JavaVersion::Java17 | JavaVersion::Java21 = version {
        if JavaVersion::Java8.get_alternate_url().is_some() {
            JavaInstallError::UnsupportedOnlyJava8
        } else {
            JavaInstallError::UnsupportedPlatform
        }
    } else {
        JavaInstallError::UnsupportedPlatform
    }
}

impl JavaVersion {
    #[must_use]
    pub(crate) fn get_alternate_url(self) -> Option<&'static str> {
        // Sources:
        // https://aws.amazon.com/corretto/
        // https://github.com/hmsjy2017/get-jdk/

        if cfg!(target_os = "linux") {
            self.get_url_linux()
        } else if cfg!(target_os = "macos") {
            self.get_url_macos()
        } else if cfg!(target_os = "windows") {
            self.get_url_windows()
        } else if cfg!(target_os = "solaris") {
            if let JavaVersion::Java8 = self {
                if cfg!(target_arch = "x86_64") {
                    Some("https://github.com/hmsjy2017/get-jdk/releases/download/v8u231/jdk-8u231-solaris-x64.tar.gz")
                } else if cfg!(target_arch = "sparc64") {
                    Some("https://github.com/hmsjy2017/get-jdk/releases/download/v8u231/jdk-8u231-solaris-sparcv9.tar.gz")
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_url_linux(self) -> Option<&'static str> {
        if cfg!(target_arch = "x86_64") {
            Some(match self {
                JavaVersion::Java16 | JavaVersion::Java17 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-17-x64-linux-jdk.zip"
                }
                JavaVersion::Java21 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-21-x64-linux-jdk.zip"
                }
                JavaVersion::Java8 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-8-x64-linux-jdk.tar.gz"
                }
            })
        } else if cfg!(target_arch = "aarch64") {
            Some(match self {
                JavaVersion::Java16 | JavaVersion::Java17 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-17-aarch64-linux-jdk.tar.gz"
                }
                JavaVersion::Java21 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-21-aarch64-linux-jdk.tar.gz"
                }
                JavaVersion::Java8 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-8-aarch64-linux-jdk.tar.gz"
                }
            })
        } else if cfg!(target_arch = "arm") {
            if let JavaVersion::Java8 = self {
                Some("https://github.com/hmsjy2017/get-jdk/releases/download/v8u231/jdk-8u231-linux-arm32-vfp-hflt.tar.gz")
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_url_macos(self) -> Option<&'static str> {
        if cfg!(target_arch = "x86_64") {
            Some(match self {
                JavaVersion::Java16 | JavaVersion::Java17 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-17-x64-macos-jdk.tar.gz"
                }
                JavaVersion::Java21 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-21-x64-macos-jdk.tar.gz"
                }
                JavaVersion::Java8 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-8-x64-macos-jdk.tar.gz"
                }
            })
        } else if cfg!(target_arch = "aarch64") {
            Some(match self {
                JavaVersion::Java16 | JavaVersion::Java17 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-17-aarch64-macos-jdk.tar.gz"
                }
                JavaVersion::Java21 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-21-aarch64-macos-jdk.tar.gz"
                }
                JavaVersion::Java8 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-8-aarch64-macos-jdk.tar.gz"
                }
            })
        } else {
            None
        }
    }

    fn get_url_windows(self) -> Option<&'static str> {
        if cfg!(target_arch = "x86_64") {
            Some(match self {
                JavaVersion::Java16 | JavaVersion::Java17 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-17-x64-windows-jdk.zip"
                }
                JavaVersion::Java21 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-21-x64-windows-jdk.zip"
                }
                JavaVersion::Java8 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-8-x64-windows-jdk.zip"
                }
            })
        } else if cfg!(target_arch = "x86") {
            Some(match self {
                JavaVersion::Java16 | JavaVersion::Java17 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-17-x86-windows-jdk.zip"
                }
                JavaVersion::Java21 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-21-x86-windows-jdk.zip"
                }
                JavaVersion::Java8 => {
                    "https://corretto.aws/downloads/latest/amazon-corretto-8-x86-windows-jdk.zip"
                }
            })
        } else {
            None
        }
    }
}

use std::{
    path::PathBuf,
    process::Stdio,
    sync::{mpsc::Sender, Arc, Mutex},
};

use ql_core::{
    file_utils, get_java_binary, info, io_err,
    json::{instance_config::InstanceConfigJson, java_list::JavaVersion, version::VersionDetails},
    JavaInstallProgress,
};
use tokio::process::{Child, Command};

use crate::ServerError;

pub async fn run_wrapped(
    name: String,
    java_install_progress: Sender<JavaInstallProgress>,
) -> Result<Arc<Mutex<Child>>, String> {
    run(&name, java_install_progress)
        .await
        .map(|n| Arc::new(Mutex::new(n)))
        .map_err(|n| n.to_string())
}

async fn run(
    name: &str,
    java_install_progress: Sender<JavaInstallProgress>,
) -> Result<Child, ServerError> {
    let launcher_dir = file_utils::get_launcher_dir()?;
    let server_dir = launcher_dir.join("servers").join(name);

    let server_jar_path = server_dir.join("server.jar");

    let version_json_path = server_dir.join("details.json");
    let version_json = tokio::fs::read_to_string(&version_json_path)
        .await
        .map_err(io_err!(version_json_path))?;
    let version_json: VersionDetails = serde_json::from_str(&version_json)?;

    let version = if let Some(version) = version_json.javaVersion.clone() {
        version.into()
    } else {
        JavaVersion::Java8
    };

    let config_json_path = server_dir.join("config.json");
    let config_json = tokio::fs::read_to_string(&config_json_path)
        .await
        .map_err(io_err!(config_json_path))?;
    let config_json: InstanceConfigJson = serde_json::from_str(&config_json)?;

    let java_path = if let Some(java_path) = &config_json.java_override {
        PathBuf::from(java_path)
    } else {
        get_java_binary(version, "java", Some(java_install_progress)).await?
    };

    let mut java_args = config_json.java_args.clone().unwrap_or_default();
    java_args.push(config_json.get_ram_argument());
    java_args.push("-jar".to_owned());
    java_args.push(server_jar_path.to_str().unwrap().to_owned());

    let mut game_args = config_json.game_args.clone().unwrap_or_default();
    game_args.push("nogui".to_owned());

    let mut command = Command::new(java_path);
    let mut command = command.args(java_args.iter().chain(game_args.iter()));

    command = if config_json.enable_logger.unwrap_or(true) {
        command
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .stdin(Stdio::piped())
    } else {
        command
    }
    .current_dir(&server_dir);

    let child = command.spawn().map_err(io_err!(server_jar_path))?;
    info!("Started server");
    Ok(child)
}

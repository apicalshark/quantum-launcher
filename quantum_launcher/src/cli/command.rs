use ql_core::{
    err, info,
    json::{InstanceConfigJson, VersionDetails},
    IntoStringError, LAUNCHER_DIR,
};
use std::io::Write;

use crate::state::get_entries;

use super::PrintCmd;

pub fn list_available_versions() {
    eprintln!("Listing downloadable versions...");
    let versions = match tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(ql_instances::list_versions())
        .strerr()
    {
        Ok(n) => n,
        Err(err) => {
            panic!("Could not list versions!\n{err}");
        }
    };

    let mut stdout = std::io::stdout().lock();
    for version in versions {
        writeln!(stdout, "{version}").unwrap();
    }
}

pub fn list_instances(cmds: &[PrintCmd], dirname: &str) {
    let instances = match tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(get_entries(dirname.to_owned(), false))
        .strerr()
    {
        Ok(n) => n.0,
        Err(err) => {
            panic!("Could not list instances: {err}");
        }
    };

    for instance in instances {
        let mut has_printed = false;
        for cmd in cmds {
            match cmd {
                PrintCmd::Name => {
                    if has_printed {
                        print!("\t");
                    }
                    print!("{instance}");
                }
                PrintCmd::Version => {
                    if has_printed {
                        print!("\t");
                    }
                    let instance_dir = LAUNCHER_DIR.join(dirname).join(&instance);

                    let json = std::fs::read_to_string(instance_dir.join("details.json")).unwrap();
                    let json: VersionDetails = serde_json::from_str(&json).unwrap();

                    print!("{}", json.id);
                }
                PrintCmd::Loader => {
                    if has_printed {
                        print!("\t");
                    }
                    let instance_dir = LAUNCHER_DIR.join(dirname).join(&instance);
                    let config_json =
                        std::fs::read_to_string(instance_dir.join("config.json")).unwrap();
                    let config_json: InstanceConfigJson =
                        serde_json::from_str(&config_json).unwrap();

                    print!("{}", config_json.mod_type);
                }
            }
            has_printed = true;
        }
        if has_printed {
            println!();
        }
    }
}

pub fn launch_instance(subcommand: (&str, &clap::ArgMatches)) {
    let instance_name: &String = subcommand.1.get_one("instance_name").unwrap();
    let username: &String = subcommand.1.get_one("username").unwrap();
    let _use_account: &bool = subcommand.1.get_one("--use-account").unwrap();
    // TODO: Implement --use-account for actual auth
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let child = match runtime.block_on(ql_instances::launch(
        instance_name.clone(),
        username.clone(),
        None,
        None,
    )) {
        Ok(n) => n,
        Err(err) => {
            err!("{err}");
            std::process::exit(1);
        }
    };

    if let (Some(stdout), Some(stderr)) = {
        let mut child = child.lock().unwrap();
        (child.stdout.take(), child.stderr.take())
    } {
        match runtime.block_on(ql_instances::read_logs(
            stdout,
            stderr,
            child,
            None,
            instance_name.clone(),
        )) {
            Ok((s, _)) => {
                info!("Game exited with code {s}");
            }
            Err(err) => {
                err!("{err}");
                std::process::exit(1);
            }
        }
    }
}

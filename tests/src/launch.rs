use std::{io::Write, time::Duration};

use ql_core::err;

use crate::{attempt, set_terminal};

pub async fn launch(name: String, timeout: f32) -> bool {
    print!("Testing {name} ");
    _ = std::io::stdout().flush();
    set_terminal(false);
    let child = attempt(
        ql_instances::launch(
            name.clone(),
            "test".to_owned(),
            None,
            None,
            None,
            Vec::new(),
        )
        .await,
    );
    set_terminal(true);

    let Some(pid) = child.child.lock().unwrap().id() else {
        err!("{name}: No PID found");
        return false;
    };

    let timeout_duration = Duration::from_secs_f32(timeout);
    let start_time = tokio::time::Instant::now();

    let sys = sysinfo::System::new_all();

    loop {
        if start_time.elapsed() >= timeout_duration {
            println!("Timeout reached!");
            break;
        }
        tokio::time::sleep(Duration::from_secs_f32(timeout / 30.0)).await;

        #[cfg(target_os = "windows")]
        todo!("Windows implementation");

        #[cfg(target_os = "macos")]
        todo!("macOS implementation");

        #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        {
            match duct::cmd("xdotool", &["search", "--pid", pid.to_string().as_str()])
                .stdout_capture()
                .run()
            {
                Ok(n) => {
                    if kill_proc(pid, &sys, n) {
                        println!();
                        return true;
                    }
                }

                Err(_) => match duct::cmd("xdotool", ["search", "--classname", "Minecraft*"])
                    .stdout_capture()
                    .run()
                {
                    Ok(n) => {
                        if kill_proc(pid, &sys, n) {
                            println!();
                            return true;
                        }
                    }
                    Err(err) if err.to_string().contains("exited with code") => {}
                    Err(err) => {
                        err!("{err:?}");
                    }
                },
            }
        }

        print!(".");
        _ = std::io::stdout().flush();
    }

    err!("{name}: No window found after waiting");
    false
}

fn kill_proc(pid: u32, sys: &sysinfo::System, n: std::process::Output) -> bool {
    if String::from_utf8_lossy(&n.stdout)
        .lines()
        .map(|n| n.trim())
        .any(|n| !n.is_empty())
    {
        for (proc_pid, proc) in sys.processes() {
            if proc_pid.as_u32() == pid {
                proc.kill();
                return true;
            }
        }
    }
    false
}

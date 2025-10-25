use std::{io::Write, time::Duration};

use ql_core::err;

use crate::{attempt, set_terminal};

use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::{EnumWindows, GetWindowThreadProcessId},
};

pub async fn launch(name: String, timeout: f32) -> bool {
    print!("Testing {name} ");
    _ = std::io::stdout().flush();
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

    let sys = sysinfo::System::new_all(); // sys needs to be mutable to refresh

    loop {
        if start_time.elapsed() >= timeout_duration {
            println!("Timeout reached!");
            break;
        }
        tokio::time::sleep(Duration::from_secs_f32(timeout / 30.0)).await;

        #[cfg(target_os = "windows")]
        {
            if search_for_window_windows(pid, &sys) {
                return true;
            }
        }

        #[cfg(target_os = "macos")]
        todo!("macOS implementation");

        #[cfg(all(target_family = "unix", not(target_os = "macos")))]
        {
            if search_for_window_unix(pid, &sys) {
                return true;
            }
        }

        print!(".");
        _ = std::io::stdout().flush();
    }

    err!("{name}: No window found after waiting");
    false
}

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
fn search_for_window_unix(pid: u32, sys: &sysinfo::System) -> bool {
    match duct::cmd("xdotool", &["search", "--pid", pid.to_string().as_str()])
        .stdout_capture()
        .run()
    {
        Ok(n) => {
            if kill_proc(pid, sys, n) {
                println!();
                return true;
            }
        }

        Err(_) => match duct::cmd("xdotool", ["search", "--classname", "Minecraft*"])
            .stdout_capture()
            .run()
        {
            Ok(n) => {
                if kill_proc(pid, sys, n) {
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
    false
}

#[cfg(all(target_family = "unix", not(target_os = "macos")))]
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

fn search_for_window_windows(pid: u32, sys: &sysinfo::System) -> bool {
    let all_windows = get_all_windows_with_pids();
    let wins: Vec<HWND> = all_windows
        .into_iter()
        .filter(|&(_, window_pid)| window_pid == pid)
        .map(|(hwnd, _)| hwnd)
        .collect();

    if wins.is_empty() {
        return false;
    }

    for (proc_pid, proc) in sys.processes() {
        if proc_pid.as_u32() == pid {
            _ = proc.kill_and_wait();
            return true;
        }
    }
    false
}

// Callback function for EnumWindows
extern "system" fn enum_windows_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {
    let window_pids = unsafe { &mut *(l_param.0 as *mut Vec<(HWND, u32)>) };

    let mut process_id: u32 = 0;
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut process_id as *mut u32));
    }

    if process_id != 0 {
        window_pids.push((hwnd, process_id));
    }

    BOOL(1) // continue enumeration
}

fn get_all_windows_with_pids() -> Vec<(HWND, u32)> {
    let mut windows_info = Vec::new();
    let ptr = &mut windows_info as *mut _;
    let l_param = ptr as isize;

    // Safety: EnumWindows is a Win32 API call. The callback handles the data.
    // The pointer passed as l_param is valid for the lifetime of the call.
    unsafe {
        EnumWindows(Some(enum_windows_callback), LPARAM(l_param))
            .expect("Failed to enumerate windows");
    }
    windows_info
}

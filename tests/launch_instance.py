import os
import re
import subprocess
import sys
import time

def run(args):
    try: subprocess.run(args)
    except subprocess.CalledProcessError as e:
        print(f"Error: Process failed with exit code {e.returncode}")
        sout = e.stdout.decode()
        if len(sout) > 0:
            print(f"Stdout:\n{sout}")
        sout = e.stderr.decode()
        if len(sout) > 0:
            print(f"Stderr:\n{sout}")
        sys.exit(1)

_ANSI_ESCAPE = re.compile(r'\x1b\[[0-9;]*[mK]')
def _remove_ansi_colors(text):
    return _ANSI_ESCAPE.sub('', text)

def _launch(instance: str) -> str | None:
    process = subprocess.Popen(
        ["target/debug/quantum_launcher", "launch", instance, "test`"],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )

    if not process.stdout:
        print("Error: Launcher has no stdout!")
        sys.exit(1)

    pid = None
    pattern = r'\[info\] Launched! PID: (\d+)'

    for line in process.stdout:
        clean_line = _remove_ansi_colors(line)
        if "No ID found!" in clean_line:
            print("Error: Game crashed instantly!")
            process.kill()
            return None
        match = re.search(pattern, clean_line)
        if match:
            pid = match.group(1)
            break

    if not pid:
        print("Error: No PID found!")
        process.kill()
        return None

    return pid

def _is_process_alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False  # No such process
    except PermissionError:
        return True   # Process exists, but not owned by you
    else:
        return True   # Process is alive

def _wait_for_window(pid: int, timeout: int) -> bool:
    start_time = time.time()
    check_interval = max(1, timeout // 30)

    while time.time() - start_time < timeout:
        if not _is_process_alive(pid):
            print("Error: Game crashed!")
            return False
        try:
            result = subprocess.check_output(["xdotool", "search", "--pid", str(pid)])
            window_ids = result.decode().strip().splitlines()
            if window_ids:
                win_id = window_ids[0]
                print(f"✅ Window found: {win_id}. Sending close input (Alt+F4)...")
                run(["xdotool", "windowactivate", win_id])
                run(["xdotool", "key", "--window", win_id, "Alt+F4"])
                return True
        except subprocess.CalledProcessError:
            pass  # No window yet
        time.sleep(check_interval)
    else:
        print("Error: Timeout waiting for window!")
        return False

def test(name: str, timeout: int) -> bool:
    pid = _launch(name)
    if pid:
        if not _wait_for_window(int(pid), timeout):
            print("Test failed (window)!")
            return False
    else:
        print("Test failed (launch)!")
        return False
    return True

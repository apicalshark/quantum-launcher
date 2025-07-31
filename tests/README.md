Command: `python -m tests`

Warning: **experimental and probably broken**,

Cli Flags (optional):
- `--help`: See help message
- `--existing`: Whether to reuse existing test files instead of redownloading them
- `--timeout`: How long to wait for a window before giving up (default: 60).
- More to come in future

This launches an instance, and closes the window if successful.
It scans for a window for `timeout` seconds.

Only works on **x11** environments (like Linux) currently.
Windows (Win32) and macOS (osascript?) support are possible,
if you're interested in helping me out,
port the `_wait_for_window` function in `launch.py`
and change the platform check in `__main__.py`.

# TODO
- Add more platform support (Windows/macOS)
- Test mod loaders?
- Also have a basic testing system for other platforms using
  crate features `simulate_*_*`. Creating instance and using `ls` and `file` to verify correct natives in `INSTANCE/libraries/natives/`

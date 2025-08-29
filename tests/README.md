Command: `python -m tests`

This launches an instance, and closes the window if successful.
It scans for a window for `timeout` seconds.

Warning: **highly experimental**

Cli Flags (optional):

- `--help`: See help message
- `--existing`: Whether to reuse existing test files instead of redownloading them
- `--timeout`: How long to wait for a window before giving up (default: 60).
- More to come in future

# Supports

- Windows
- X11 (or XWayland under Wayland) environments like **Linux, FreeBSD**, etc.

macOS (osascript?) support is possible,
if you're interested in helping me out,
port the `_wait_for_window` function in `launch.py`
and change the platform check in `__main__.py`.

# TODO

- Add macOS support
- Test mod loaders?
- Also have a basic testing system for other platforms using
  crate features `simulate_*_*`, by creating instance and using
  `ls` and `file` to verify correct natives in
  `INSTANCE/libraries/natives/`

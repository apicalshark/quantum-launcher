Run `python -m tests --instance <INSTANCE_NAME>`

It's **experimental and a basic prototype**,
just testing the waters. It doesn't really work as an
automated test right now but I will flesh it out in the future.

Only works on **Linux x11** currently.
If possible, please help making it cross platform
(contributions are welcome)!

Cli Arguments:
- `--instance`: (required) Which instance to open and test
- `--timeout`: How long to wait for a window before giving up (default: 60).
- More to come in future

This launches an instance, and closes the window if successful.
It scans for a window for `timeout` seconds.

# TODO
- Automatically create instances to test
  (requires adding launcher CLI for creating instances,
  and filtering `list-available-versions`)
- Sandbox the execution into a disposable temp directory
  using qldir.txt: [More Info](https://github.com/Mrmayman/quantumlauncher/tree/main/docs/PORTABLE_MODE.md)
  **NOTE: Make sure to add this temp dir to gitignore and auto clean it up**
- Make multiple instances of this test suite launch in parallel

- Also have a basic testing system for other platforms using
  crate features `simulate_*_*`. Creating instance and using `ls` and `file` to verify correct natives in `INSTANCE/libraries/natives/`

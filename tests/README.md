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

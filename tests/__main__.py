import os
import sys
import argparse

from . import launch_instance

if os.getenv("XDG_SESSION_TYPE") != "x11":
    print("Unsupported platform!\nThis test suite is only runnable on Linux x11")
    sys.exit(1)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--timeout", type=int, default=60, help="Timeout in seconds (default: 60)")
    parser.add_argument("--instance", required=True, help="Instance ID to test")
    args = parser.parse_args()

    launch_instance.run(["cargo", "build"])
    if not launch_instance.test(args.instance, args.timeout):
        sys.exit(1)

if __name__ == "__main__":
    main()

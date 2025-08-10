# Unreleased

# Switch to BetterJSONs and LaunchWrapper

- The launcher now uses [BetterJSONs](https://github.com/MCPHackers/BetterJSONs/)
  and [LaunchWrapper](https://github.com/MCPHackers/LaunchWrapper) internally for managing Minecraft versions
- This one change has fixed **so much**, everything as a result of this will be marked with (b).

---

- Overhauled portable dir system (see `docs/PORTABLE.md` for more info)
- Added a new Teal color scheme!
- Overhauled the Launcher Settings menu

# [`ely.by`](https://ely.by)/[`littleskin`](https://littleskin.cn) integration

- You can now log in with [`ely.by`](https://ely.by) and [`littleskin`](https://littleskin.cn) accounts!
- Minecraft 1.21.5 and below support skins from both services (b)

> **Note**:
> You'll need to create a new instance for skins to work without mods.
>
> For existing instances, and for 1.21.6+, use the CustomSkinLoader mod

# UI

- Added a licenses page

## Revamped:

- Launcher settings
- Confirmation screen
- All launcher icons (thanks, [Aurlt](https://github.com/Aurlt) !)

## Improved:

- Instance edit menu

# Tweaks

- You can now change antialiasing for the UI in settings
- The launcher now remembers the last account selected upon opening

# CLI

The following commands have been added:

- `quantum_launcher create <NAME> <VERSION>` (add `-s` to skip downloading assets (music/sound))
- `quantum_launcher launch <INSTANCE> <USERNAME>` (add `-s` for account authentication)
- `quantum_launcher delete <INSTANCE>` (add `-f` to skip confirmation)

# Platform

Added (incomplete) support for:

- FreeBSD
- Windows 7
- Linux ARM 32-bit

# Fixes

- Fixed Minecraft Indev and early Infdev being unplayable (b)
- Fixed many crashes on Linux ARM and macOS (b)
- Fixed broken colors in old versions on M-series Macs (b)
- Fixed getting stuck in an infinite loop when downloading some curseforge mods
- Fixed Fabric API being missing for some curseforge mods
- Fixed game crashes in portable mode
- Fixed java install progress bar being stuck at the end
- Fixed many formatting issues in game logs
- Fixed welcome screen not working
- Fixed arrow keys to switch instances, not updating the Edit menu

- Old Minecraft versions are now in the correct order in the download list (b)
- Snapshots of 1.0 to 1.5.2 are no longer missing for download (b)
- Improved readability of a few errors
- Improved support for weird character encodings in file paths
- Missing libraries are now auto-downloaded

## Performance

- Fixed lag spikes on some systems when selecting instances
- Many autosaving features has been slowed down, and disk accesses reduced
- Optimized the log renderer (slightly worse scrolling as a tradeoff)
- The "Create Instance" version list loads **way** faster now (b)

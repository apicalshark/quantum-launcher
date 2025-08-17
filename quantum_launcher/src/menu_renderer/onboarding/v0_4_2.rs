use iced::widget;

use crate::{
    menu_renderer::{link, Element, FONT_MONO},
    state::Message,
    stylesheet::styles::LauncherTheme,
};

#[allow(unused)]
pub fn changelog<'a>() -> Element<'a> {
    const FS: u16 = 14;

    widget::column![
        widget::text("Welcome to QuantumLauncher v0.4.2!").size(40),
        e_tldr(),
        widget::Space::with_height(5),

        e_login(),

        widget::horizontal_rule(1),
        widget::text("Other:").size(32),
        widget::row![
            "- Logs can now be uploaded from within the launcher, to ",
            link("mclo.gs", "https://mclo.gs".to_owned()),
            " for easy sharing"
        ].wrap(),
        "- You can now set Minecraft's window size both globally and per-instance",
        "- The launcher now retains its own window size from the previous session (this feature can be disabled)",

        widget::horizontal_rule(1),
        widget::text("Revamped Menus:").size(32),
        "- Launcher settings (redesign + licenses page + UI antialiasing option)",
        "- Choice confirmation screen (eg. delete)",
        widget::row![
            "- All launcher icons (thanks, ",
            link("Aurlt", "https://github.com/Aurlt".to_owned()),
            " !)"
        ].wrap(),
        "- OptiFine install menu (now with Drag & Drop, delete installer option)",

        e_cli(),

        widget::horizontal_rule(1),
        widget::text("System & Platform").size(32),

        widget::column![
            "- Overhauled portable/custom directory system",
            "- Linux ARM 32-bit is now supported!",
            "- Experimental FreeBSD support is also available",
        ].spacing(5),

        widget::horizontal_rule(1),
        widget::column![
            widget::text("Fixes").size(32),
            widget::text("- Fixed many crashes on Linux ARM and macOS `(b)`").size(FS),
            widget::text("- Fixed game crashes in portable mode").size(FS),
            widget::text("- Fixed many formatting issues in game logs").size(FS),
            widget::text("- Fixed welcome screen not working").size(FS),
            widget::text("- Fixed arrow keys to switch instances, not updating the Edit menu").size(FS),
            widget::Space::with_height(5),
            widget::text("- Improved readability of a few errors").size(FS),
            widget::text("- Improved support for weird character encodings in file paths").size(FS),
            widget::text("- Missing libraries are now auto-downloaded").size(FS),
            widget::text("- Last account selected is now remembered").size(FS),
            widget::text("- On Windows, custom file managers are now respected when opening files").size(FS),
            widget::Space::with_height(5),
            widget::text("- Added dialog to prevent glitching upon changing UI scale").size(FS),
            widget::text("- Removed Ctrl-Scroll to change UI scale due to many bugs").size(FS),
        ].spacing(5),

        widget::column![
            widget::text("Modding:").size(20),
            widget::text("- Fixed Fabric API being missing for some curseforge mods").size(FS),
            widget::text("- Fixed getting stuck in an infinite loop when downloading some curseforge mods").size(FS),
            widget::text("- Fixed modrinth mods repeating infinitely in the store list").size(FS),
            widget::text("- Improved mod description rendering in the store").size(FS),
        ].spacing(5),

        widget::column![
            widget::text("Versions:").size(20),
            widget::text("- Fixed Minecraft Indev and early Infdev being unplayable (b)").size(FS),
            widget::text("- Fixed broken colors in old versions on M-series Macs (b)").size(FS),
            widget::text("- Old Minecraft versions are now in the correct order in the download list (b)").size(FS),
            widget::text("- Snapshots of 1.0 to 1.5.2 are no longer missing for download (b)").size(FS),
        ].spacing(5),

        widget::column![
            widget::text("Performance:").size(20),
            widget::text("- Fixed lag spikes on some systems when selecting instances").size(FS),
            widget::text("- Many autosaving features has been slowed down, and disk accesses reduced").size(FS),

            widget::Space::with_height(5),
            widget::text("Optimized:"),
            widget::text("- \"Create Instance\" version list `(b)`").size(FS),
            widget::text("- Recommended mods list").size(FS),
            widget::text("- Forge installation for older versions").size(FS),
            widget::text("- Log renderer (slightly worse scrolling as a tradeoff)").size(FS),
        ].spacing(5),

        widget::horizontal_rule(1),
        widget::text("...and it's all here!"),
        widget::text("Ready to experience it now? Hit continue!").size(20),
    ]
    .padding(10)
    .spacing(10)
    .into()
}

fn e_cli<'a>() -> widget::Column<'a, Message, LauncherTheme> {
    const INDENT: u16 = 50;

    widget::column![
        widget::horizontal_rule(1),
        widget::text("CLI:").size(32),
        "The following terminal commands have been added:",
        widget::column![
            widget::container(
                widget::text("quantum_launcher create <NAME> <VERSION>").font(FONT_MONO)
            )
            .padding(2),
            widget::row![
                widget::Space::with_width(INDENT),
                "Add -s to skip downloading assets (music/sound)"
            ]
            .wrap(),
            widget::container(
                widget::text("quantum_launcher launch <INSTANCE> <USERNAME>").font(FONT_MONO)
            )
            .padding(2),
            widget::row![
                widget::Space::with_width(INDENT),
                "Add -s for account authentication"
            ]
            .wrap(),
            widget::container(widget::text("quantum_launcher delete <INSTANCE>").font(FONT_MONO))
                .padding(2),
            widget::row![
                widget::Space::with_width(INDENT),
                "Add -f to skip confirmation"
            ]
            .wrap(),
        ]
        .spacing(5)
    ]
    .spacing(10)
}

fn e_login<'a>() -> widget::Column<'a, Message, LauncherTheme> {
    widget::column![
        widget::row![
            link(widget::text("ely.by").size(32), "https://ely.by".to_owned()),
            widget::text(" and ").size(32),
            link(
                widget::text("littleskin").size(32),
                "https://littleskin.cn".to_owned()
            ),
            widget::text(" integration").size(32),
        ]
        .wrap(),
        widget::column![
            "- You can now log in with ely.by and littleskin accounts!",
            "- Minecraft 1.21.5 and below support skins from both services (b)",
        ]
        .spacing(5),
        widget::container(
            widget::column![
                "Note: You'll need to create a new instance for skins to work without mods.",
                "For existing instances, and for 1.21.6+, use the CustomSkinLoader mod"
            ]
            .padding(10)
            .spacing(10)
        )
    ]
    .spacing(10)
}

fn e_tldr<'a>() -> widget::Container<'a, Message, LauncherTheme> {
    widget::container(
        widget::column![
            widget::text("Revamped and improved many menus, plus a new Teal theme!"),
            widget::text("Added ely.by and littleskin.cn account/skin integration!"),
            widget::row![
                "- Switched to ",
                link(
                    "BetterJSONs",
                    "https://github.com/MCPHackers/BetterJSONs".to_owned()
                ),
                " and ",
                link(
                    "LaunchWrapper",
                    "https://github.com/MCPHackers/LaunchWrapper".to_owned()
                ),
                ",",
                " fixing many issues (marked with (b))"
            ]
            .wrap(),
            widget::text("- Many, MANY bug-fixes and performance improvements!"),
        ]
        .padding(10)
        .spacing(5),
    )
}

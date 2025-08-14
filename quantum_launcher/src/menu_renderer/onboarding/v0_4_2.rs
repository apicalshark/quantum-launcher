use iced::widget;

use crate::{
    menu_renderer::{link, Element, FONT_MONO},
    state::Message,
    stylesheet::styles::LauncherTheme,
};

#[allow(unused)]
pub fn changelog<'a>() -> Element<'a> {
    const INDENT: u16 = 50;
    const FS: u16 = 14;

    widget::column![
        widget::text("Welcome to QuantumLauncher v0.4.2!").size(40),
        tldr(),
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

        widget::horizontal_rule(1),
        widget::text("CLI:").size(32),
        "The following terminal commands have been added:",

        widget::column![
            widget::container(widget::text("quantum_launcher create <NAME> <VERSION>").font(FONT_MONO)).padding(2),
            widget::row![
                widget::Space::with_width(INDENT),
                "Add -s to skip downloading assets (music/sound)"
            ].wrap(),

            widget::container(widget::text("quantum_launcher launch <INSTANCE> <USERNAME>").font(FONT_MONO)).padding(2),
            widget::row![
                widget::Space::with_width(INDENT),
                "Add -s for account authentication"
            ].wrap(),

            widget::container(widget::text("quantum_launcher delete <INSTANCE>").font(FONT_MONO)).padding(2),
            widget::row![
                widget::Space::with_width(INDENT),
                "Add -f to skip confirmation"
            ].wrap(),
        ].spacing(5),

        widget::horizontal_rule(1),
        widget::text("System & Platform").size(32),

        widget::column![
            "- Overhauled portable/custom directory system",
            "- Linux ARM 32-bit is now supported!",
            "- Experimental FreeBSD support is also available",
        ].spacing(5),

        widget::horizontal_rule(1),
        widget::text("Fixes").size(32),

        widget::column![
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
        ].spacing(5),

        widget::text("- Modding").size(32),

    ]
    .padding(10)
    .spacing(10)
    .into()
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

fn tldr<'a>() -> widget::Container<'a, Message, LauncherTheme> {
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

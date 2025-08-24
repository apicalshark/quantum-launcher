use std::collections::HashSet;

use iced::{widget, Length};
use ql_core::SelectedMod;

use crate::{
    icon_manager,
    menu_renderer::{back_button, button_with_icon, Element},
    state::{
        EditPresetsMessage, ManageModsMessage, MenuEditPresets, MenuRecommendedMods, Message,
        ModListEntry, SelectedState,
    },
    stylesheet::{color::Color, styles::LauncherTheme},
};

impl MenuEditPresets {
    pub fn view(&'_ self) -> Element<'_> {
        if let Some(progress) = &self.progress {
            return widget::column!(
                widget::text("Installing mods").size(20),
                progress.view(),
                widget::text("Check debug log (at the bottom) for more info").size(12),
            )
            .padding(10)
            .spacing(10)
            .into();
        }

        if self.is_building {
            return widget::column!(widget::text("Building Preset").size(20))
                .padding(10)
                .spacing(10)
                .into();
        }

        let p_main = widget::scrollable(
            widget::column![
                widget::row![
                    back_button().on_press(Message::ManageMods(
                        ManageModsMessage::ScreenOpenWithoutUpdate
                    )),
                    widget::tooltip(
                        button_with_icon(icon_manager::folder_with_size(14), "Import", 14)
                            .on_press(Message::EditPresets(EditPresetsMessage::Load)),
                        widget::column![
                            widget::text("Note: Sideloaded .jar mods in untrusted presets could have viruses").size(12),
                            widget::text("To get rid of them, after installing remove all mods in the list ending in \".jar\"").size(12)
                        ],
                        widget::tooltip::Position::Bottom
                    )
                    .style(|n: &LauncherTheme| n.style_container_sharp_box(0.0, Color::ExtraDark)),
                ].spacing(5),
                self.get_page_contents()
            ]
            .padding(10)
            .spacing(10)
            .width(Length::Fill)
        ).style(|t: &LauncherTheme, s| t.style_scrollable_flat_dark(s));

        if self.drag_and_drop_hovered {
            widget::stack!(
                p_main,
                widget::center(widget::button(
                    widget::text("Drag and drop mod files to add them").size(20)
                ))
            )
            .into()
        } else {
            p_main.into()
        }
    }

    fn get_page_contents(&'_ self) -> Element<'_> {
        widget::column!(
            widget::text("Share and import a collection of mods!")
                .size(14)
                .style(|t: &LauncherTheme| t.style_text(Color::SecondLight)),
            if self.sorted_mods_list.is_empty() {
                widget::column![
                    "You have no mods installed!",
                    widget::button("View Recommended Mods").on_press(Message::RecommendedMods(
                        crate::state::RecommendedModMessage::Open
                    ))
                ]
            } else {
                widget::column![
                    widget::text("Create Modpack").size(20),
                    "Select Mods to keep",
                    widget::button(if let SelectedState::All = self.selected_state {
                        "Unselect All"
                    } else {
                        "Select All"
                    })
                    .on_press(Message::EditPresets(EditPresetsMessage::SelectAll)),
                    widget::container(self.get_mods_list(&self.selected_mods).padding(10)),
                    button_with_icon(icon_manager::save(), "Build Preset", 16)
                        .on_press(Message::EditPresets(EditPresetsMessage::BuildYourOwn)),
                ]
            }
            .spacing(10)
        )
        .spacing(10)
        .into()
    }

    fn get_mods_list<'a>(
        &'a self,
        selected_mods: &'a HashSet<SelectedMod>,
    ) -> widget::Column<'a, Message, LauncherTheme, iced::Renderer> {
        widget::column(self.sorted_mods_list.iter().map(|entry| {
            if entry.is_manually_installed() {
                widget::checkbox(entry.name(), selected_mods.contains(&entry.id()))
                    .on_toggle(move |t| match entry {
                        ModListEntry::Downloaded { id, config } => {
                            Message::EditPresets(EditPresetsMessage::ToggleCheckbox(
                                (config.name.clone(), id.clone()),
                                t,
                            ))
                        }
                        ModListEntry::Local { file_name } => Message::EditPresets(
                            EditPresetsMessage::ToggleCheckboxLocal(file_name.clone(), t),
                        ),
                    })
                    .into()
            } else {
                widget::text!(" - (DEPENDENCY) {}", entry.name()).into()
            }
        }))
        .spacing(5)
    }
}

impl MenuRecommendedMods {
    pub fn view(&'_ self) -> Element<'_> {
        let back_button = back_button().on_press(Message::ManageMods(
            ManageModsMessage::ScreenOpenWithoutUpdate,
        ));

        match self {
            MenuRecommendedMods::Loading { progress, .. } => progress.view().padding(10).into(),
            MenuRecommendedMods::InstallALoader => {
                widget::column![
                    back_button,
                    "Install a mod loader (like Fabric/Forge/NeoForge/Quilt/etc, whichever is compatible)",
                    "You need one before you can install mods"
                ].padding(10).spacing(5).into()
            }
            MenuRecommendedMods::NotSupported => {
                widget::column![
                    back_button,
                    "No recommended mods found :)"
                ].padding(10).spacing(5).into()
            }
            MenuRecommendedMods::Loaded { mods, .. } => {
                let content: Element =
                    widget::column!(
                        back_button,
                        button_with_icon(icon_manager::download(), "Download Recommended Mods", 16)
                            .on_press(Message::RecommendedMods(
                                crate::state::RecommendedModMessage::Download
                            )),
                        widget::column(mods.iter().enumerate().map(|(i, (e, n))| {
                            let elem: Element = widget::checkbox(n.name, *e)
                                .on_toggle(move |n| {
                                    Message::RecommendedMods(crate::state::RecommendedModMessage::Toggle(
                                        i, n,
                                    ))
                                })
                                .into();
                            widget::column!(elem, widget::text(n.description).size(12))
                                .spacing(5)
                                .into()
                        }))
                        .spacing(10)
                    )
                    .spacing(10)
                    .into();

                widget::scrollable(widget::column![content].padding(10))
                    .style(|t: &LauncherTheme, status| t.style_scrollable_flat_dark(status))
                    .into()
            }
        }
    }
}

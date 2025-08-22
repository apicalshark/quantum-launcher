use iced::{widget, Length};
use ql_core::{SelectedMod, ModId};

use crate::{
    icon_manager,
    menu_renderer::{back_button, button_with_icon, Element},
    state::{ExportModsMessage, ImageState, ManageModsMessage, MenuExportMods, Message},
    stylesheet::{color::Color, styles::LauncherTheme},
};

impl MenuExportMods {
    pub fn view(&self, images: &ImageState, window_size: (f32, f32)) -> Element {
        let selected_count = self.selected_mods.len();
        let has_mods = selected_count > 0;
        
        widget::container(
            widget::scrollable(
                widget::column![
                    // Header with back button
                    widget::row![
                        back_button().on_press(Message::ManageMods(ManageModsMessage::ScreenOpen)),
                        widget::text("Export Mods").size(24).style(|theme: &LauncherTheme| {
                            theme.style_text(Color::Light)
                        }),
                    ]
                    .spacing(15)
                    .align_y(iced::Alignment::Center),
                    
                    // Info about selected mods with better styling
                    widget::container(
                        widget::text(if has_mods {
                            format!("{} mod{} selected for export", 
                                selected_count,
                                if selected_count == 1 { "" } else { "s" }
                            )
                        } else {
                            "No mods selected - please select some mods first".to_string()
                        })
                        .size(16)
                        .style(move |theme: &LauncherTheme| {
                            if has_mods {
                                theme.style_text(Color::SecondLight)
                            } else {
                                theme.style_text(Color::SecondDark)
                            }
                        })
                    )
                    .padding([10, 15])
                    .style(|theme: &LauncherTheme| {
                        theme.style_container_sharp_box(0.0, Color::ExtraDark)
                    }),
                    
                    // Export options with better spacing
                    widget::column![
                        widget::text("Choose export format:")
                            .size(18)
                            .style(|theme: &LauncherTheme| {
                                theme.style_text(Color::Light)
                            }),
                        
                        // Export as Plain Text - improved styling
                        widget::container(
                            widget::row![
                                widget::container(
                                    button_with_icon(icon_manager::text_file_with_size(28), "", 28)
                                )
                                .padding(5),
                                widget::column![
                                    widget::text("Export as Plain Text")
                                        .size(17)
                                        .style(|theme: &LauncherTheme| {
                                            theme.style_text(Color::Light)
                                        }),
                                    widget::text("Simple text file with mod names, one per line")
                                        .size(13)
                                        .style(|theme: &LauncherTheme| {
                                            theme.style_text(Color::SecondLight)
                                        }),
                                ]
                                .spacing(4),
                                widget::horizontal_space(),
                                widget::row![
                                    {
                                        let copy_button = widget::button(
                                            widget::text("Copy").size(14)
                                        )
                                        .padding([8, 16])
                                        .style(|theme: &LauncherTheme, status| {
                                            use crate::stylesheet::widgets::StyleButton;
                                            theme.style_button(status, StyleButton::Round)
                                        });
                                        
                                        if has_mods {
                                            copy_button.on_press(Message::ExportMods(ExportModsMessage::CopyPlainTextToClipboard))
                                        } else {
                                            copy_button
                                        }
                                    },
                                    {
                                        let save_button = widget::button(
                                            widget::text("Save").size(14)
                                        )
                                        .padding([8, 16])
                                        .style(|theme: &LauncherTheme, status| {
                                            use crate::stylesheet::widgets::StyleButton;
                                            theme.style_button(status, StyleButton::FlatDark)
                                        });
                                        
                                        if has_mods {
                                            save_button.on_press(Message::ExportMods(ExportModsMessage::ExportAsPlainText))
                                        } else {
                                            save_button
                                        }
                                    }
                                ]
                                .spacing(12)
                            ]
                            .spacing(20)
                            .align_y(iced::Alignment::Center)
                            .padding(20)
                        )
                        .style(|theme: &LauncherTheme| {
                            theme.style_container_sharp_box(0.0, Color::Dark)
                        }),
                        
                        // Export as Markdown - improved styling
                        widget::container(
                            widget::row![
                                widget::container(
                                    button_with_icon(icon_manager::text_file_with_size(28), "", 28)
                                )
                                .padding(5),
                                widget::column![
                                    widget::text("Export as Markdown")
                                        .size(17)
                                        .style(|theme: &LauncherTheme| {
                                            theme.style_text(Color::Light)
                                        }),
                                    widget::text("Markdown file with clickable mod links")
                                        .size(13)
                                        .style(|theme: &LauncherTheme| {
                                            theme.style_text(Color::SecondLight)
                                        }),
                                ]
                                .spacing(4),
                                widget::horizontal_space(),
                                widget::row![
                                    {
                                        let copy_button = widget::button(
                                            widget::text("Copy").size(14)
                                        )
                                        .padding([8, 16])
                                        .style(|theme: &LauncherTheme, status| {
                                            use crate::stylesheet::widgets::StyleButton;
                                            theme.style_button(status, StyleButton::Round)
                                        });
                                        
                                        if has_mods {
                                            copy_button.on_press(Message::ExportMods(ExportModsMessage::CopyToClipboard))
                                        } else {
                                            copy_button
                                        }
                                    },
                                    {
                                        let save_button = widget::button(
                                            widget::text("Save").size(14)
                                        )
                                        .padding([8, 16])
                                        .style(|theme: &LauncherTheme, status| {
                                            use crate::stylesheet::widgets::StyleButton;
                                            theme.style_button(status, StyleButton::FlatDark)
                                        });
                                        
                                        if has_mods {
                                            save_button.on_press(Message::ExportMods(ExportModsMessage::ExportAsMarkdown))
                                        } else {
                                            save_button
                                        }
                                    }
                                ]
                                .spacing(12)
                            ]
                            .spacing(20)
                            .align_y(iced::Alignment::Center)
                            .padding(20)
                        )
                        .style(|theme: &LauncherTheme| {
                            theme.style_container_sharp_box(0.0, Color::Dark)
                        }),
                        
                        // Preview section with better styling - only show if mods are selected
                        {
                            if has_mods {
                                widget::column![
                                    widget::text("Preview:")
                                        .size(18)
                                        .style(|theme: &LauncherTheme| {
                                            theme.style_text(Color::Light)
                                        }),
                                    widget::container(
                                        widget::scrollable(self.get_preview_content(images, window_size))
                                            .width(Length::Fill)
                                            .height(400) // Fixed height that provides plenty of space
                                    )
                                    .style(|theme: &LauncherTheme| {
                                        theme.style_container_sharp_box(0.0, Color::ExtraDark)
                                    })
                                    .padding(15)
                                    .width(Length::Fill),
                                ]
                                .spacing(10)
                            } else {
                                widget::column![
                                    widget::text("Preview:")
                                        .size(18)
                                        .style(|theme: &LauncherTheme| {
                                            theme.style_text(Color::Light)
                                        }),
                                    widget::container(
                                        widget::text("Select some mods from the mod manager to see a preview here")
                                            .size(14)
                                            .style(|theme: &LauncherTheme| {
                                                theme.style_text(Color::SecondLight)
                                            })
                                    )
                                    .center_x(Length::Fill)
                                    .center_y(Length::Fixed(200.0))
                                    .style(|theme: &LauncherTheme| {
                                        theme.style_container_sharp_box(0.0, Color::ExtraDark)
                                    })
                                    .padding(20)
                                    .height(400)
                                    .width(Length::Fill),
                                ]
                                .spacing(10)
                            }
                        }
                    ]
                    .spacing(20)
                ]
                .spacing(25)
                .padding(25)
            )
            .style(LauncherTheme::style_scrollable_flat_dark)
        )
        .style(|theme: &LauncherTheme| {
            theme.style_container_sharp_box(0.0, Color::Dark)
        })
        .into()
    }
    
    fn get_preview_content(&self, _images: &ImageState, _window_size: (f32, f32)) -> Element {
        if self.selected_mods.is_empty() {
            return widget::container(
                widget::column![
                    widget::text("No mods selected")
                        .size(16)
                        .style(|theme: &LauncherTheme| {
                            theme.style_text(Color::SecondLight)
                        }),
                    widget::text("Select some mods from the mod manager to export them")
                        .size(12)
                        .style(|theme: &LauncherTheme| {
                            theme.style_text(Color::SecondLight)
                        })
                ]
                .spacing(5)
                .align_x(iced::Alignment::Center)
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }
        
        let mut preview_elements = Vec::new();
        
        // Add a header to show this is a preview
        preview_elements.push(
            widget::text("Mod List Preview (showing clickable links)")
                .size(13)
                .style(|theme: &LauncherTheme| {
                    theme.style_text(Color::SecondLight)
                })
                .into()
        );
        
        // Add separator
        preview_elements.push(
            widget::container(widget::text(""))
                .height(1)
                .width(Length::Fill)
                .style(|theme: &LauncherTheme| {
                    theme.style_container_sharp_box(0.0, Color::SecondDark)
                })
                .into()
        );
        
        for (index, selected_mod) in self.selected_mods.iter().take(10).enumerate() {
            match selected_mod {
                SelectedMod::Downloaded { name, id } => {
                    let url = if let Some(mod_config) = self.mod_index.mods.get(&id.get_index_str()) {
                        match mod_config.project_source.as_str() {
                            "modrinth" => {
                                format!("https://modrinth.com/mod/{}", mod_config.project_id)
                            }
                            "curseforge" => {
                                format!("https://www.curseforge.com/minecraft/mc-mods/{}", mod_config.project_id)
                            }
                            _ => {
                                match id {
                                    ModId::Modrinth(mod_id) => {
                                        format!("https://modrinth.com/mod/{}", mod_id)
                                    }
                                    ModId::Curseforge(mod_id) => {
                                        format!("https://www.curseforge.com/minecraft/mc-mods/{}", mod_id)
                                    }
                                }
                            }
                        }
                    } else {
                        match id {
                            ModId::Modrinth(mod_id) => {
                                format!("https://modrinth.com/mod/{}", mod_id)
                            }
                            ModId::Curseforge(mod_id) => {
                                format!("https://www.curseforge.com/minecraft/mc-mods/{}", mod_id)
                            }
                        }
                    };
                    
                    // Create a clickable link that looks like markdown with better styling
                    let link_element = widget::container(
                        widget::button(
                            widget::row![
                                widget::text(format!("{}.", index + 1))
                                    .size(13)
                                    .style(|theme: &LauncherTheme| {
                                        theme.style_text(Color::SecondLight)
                                    }),
                                widget::text(name)
                                    .size(13)
                                    .style(|theme: &LauncherTheme| {
                                        theme.style_text(Color::Light)
                                    }),
                                widget::text("→")
                                    .size(13)
                                    .style(|theme: &LauncherTheme| {
                                        theme.style_text(Color::SecondLight)
                                    })
                            ]
                            .align_y(iced::Alignment::Center)
                            .spacing(8)
                        )
                        .style(|theme: &LauncherTheme, status| {
                            use crate::stylesheet::widgets::StyleButton;
                            theme.style_button(status, StyleButton::Flat)
                        })
                        .on_press(Message::CoreOpenLink(url))
                    )
                    .padding([5, 0])
                    .width(Length::Fill);
                    
                    preview_elements.push(link_element.into());
                }
                SelectedMod::Local { file_name } => {
                    let display_name = file_name
                        .strip_suffix(".jar")
                        .or_else(|| file_name.strip_suffix(".zip"))
                        .unwrap_or(file_name.as_str());
                    
                    let text_element = widget::container(
                        widget::row![
                            widget::text(format!("{}.", index + 1))
                                .size(13)
                                .style(|theme: &LauncherTheme| {
                                    theme.style_text(Color::SecondLight)
                                }),
                            widget::text(display_name)
                                .size(13)
                                .style(|theme: &LauncherTheme| {
                                    theme.style_text(Color::Light)
                                }),
                            widget::text("(local)")
                                .size(12)
                                .style(|theme: &LauncherTheme| {
                                    theme.style_text(Color::SecondLight)
                                })
                        ]
                        .align_y(iced::Alignment::Center)
                        .spacing(8)
                    )
                    .padding([5, 0])
                    .width(Length::Fill);
                    
                    preview_elements.push(text_element.into());
                }
            }
        }
        
        // Add message if there are more mods than shown
        if self.selected_mods.len() > 10 {
            preview_elements.push(
                widget::text(format!("... and {} more mod{}", 
                    self.selected_mods.len() - 10,
                    if self.selected_mods.len() - 10 == 1 { "" } else { "s" }
                ))
                .size(12)
                .style(|theme: &LauncherTheme| {
                    theme.style_text(Color::SecondLight)
                })
                .into()
            );
        }
        
        widget::column(preview_elements)
            .spacing(6)
            .padding(10)
            .into()
    }
}

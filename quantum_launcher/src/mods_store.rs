use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use iced::Command;
use ql_core::{
    file_utils, io_err,
    json::{instance_config::InstanceConfigJson, version::VersionDetails},
    InstanceSelection,
};
use ql_mod_manager::mod_manager::{Loader, ModIndex, Query, Search};

use crate::launcher_state::{Launcher, MenuModsDownload, Message, State};

impl Launcher {
    pub fn open_mods_screen(&mut self) -> Result<Command<Message>, String> {
        let selection = self.selected_instance.as_ref().unwrap();
        let instances_dir =
            file_utils::get_instance_dir(selection).map_err(|err| err.to_string())?;

        let config_path = instances_dir.join("config.json");
        let config = std::fs::read_to_string(&config_path)
            .map_err(io_err!(config_path))
            .map_err(|err| err.to_string())?;
        let config: InstanceConfigJson =
            serde_json::from_str(&config).map_err(|err| err.to_string())?;

        let version_path = instances_dir.join("details.json");
        let version = std::fs::read_to_string(&version_path)
            .map_err(io_err!(version_path))
            .map_err(|err| err.to_string())?;
        let version: VersionDetails =
            serde_json::from_str(&version).map_err(|err| err.to_string())?;

        let mod_index = ModIndex::get(&selection).map_err(|n| n.to_string())?;

        let mut menu = MenuModsDownload {
            config,
            json: Box::new(version),
            is_loading_search: false,
            latest_load: Instant::now(),
            query: String::new(),
            results: None,
            opened_mod: None,
            result_data: HashMap::new(),
            mods_download_in_progress: HashSet::new(),
            mod_index,
        };
        let command = menu.search_modrinth(matches!(
            &self.selected_instance,
            Some(InstanceSelection::Server(_))
        ));
        self.state = State::ModsDownload(Box::new(menu));
        Ok(command)
    }
}

impl MenuModsDownload {
    pub fn search_modrinth(&mut self, is_server: bool) -> Command<Message> {
        let Some(loaders) = (match self.config.mod_type.as_str() {
            "Forge" => Some(vec![Loader::Forge]),
            "Fabric" => Some(vec![Loader::Fabric]),
            _ => None,
        }) else {
            return Command::none();
        };

        self.is_loading_search = true;
        Command::perform(
            Search::search_w(Query {
                name: self.query.clone(),
                versions: vec![self.json.id.clone()],
                loaders,
                server_side: is_server,
                open_source: false, // TODO: Add Open Source filter
            }),
            Message::InstallModsSearchResult,
        )
    }
}

use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    process::ExitStatus,
    sync::{mpsc::Receiver, Arc, Mutex},
    time::Instant,
};

use iced::{widget::image::Handle, Command};
use lazy_static::lazy_static;
use ql_core::{
    err, file_utils, info,
    json::{instance_config::InstanceConfigJson, version::VersionDetails},
    pt, DownloadProgress, GenericProgress, InstanceSelection, IntoIoError, JsonFileError, Progress,
    SelectedMod, LAUNCHER_VERSION_NAME,
};
use ql_instances::{GameLaunchResult, ListEntry, LogLine, UpdateCheckInfo};
use ql_mod_manager::{
    loaders::{
        fabric::FabricVersionListItem, forge::ForgeInstallProgress,
        optifine::OptifineInstallProgress,
    },
    mod_manager::{ImageResult, Loader, ModConfig, ModIndex, ProjectInfo, RecommendedMod, Search},
};
use tokio::process::{Child, ChildStdin};

use crate::{
    config::LauncherConfig,
    message_handler::{get_locally_installed_mods, open_file_explorer},
    stylesheet::styles::{LauncherStyle, LauncherTheme, STYLE},
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

#[derive(Debug, Clone)]
pub enum InstallFabricMessage {
    End(Result<bool, String>),
    VersionSelected(String),
    VersionsLoaded(Result<Vec<FabricVersionListItem>, String>),
    ButtonClicked,
    ScreenOpen { is_quilt: bool },
}

#[derive(Debug, Clone)]
pub enum CreateInstanceMessage {
    ScreenOpen,
    VersionsLoaded(Result<Vec<ListEntry>, String>),
    VersionSelected(ListEntry),
    NameInput(String),
    Start,
    End(Result<String, String>),
    ChangeAssetToggle(bool),
}

#[derive(Debug, Clone)]
pub enum EditInstanceMessage {
    MenuOpen,
    JavaOverride(String),
    MemoryChanged(f32),
    LoggingToggle(bool),
    JavaArgsAdd,
    JavaArgEdit(String, usize),
    JavaArgDelete(usize),
    JavaArgShiftUp(usize),
    JavaArgShiftDown(usize),
    GameArgsAdd,
    GameArgEdit(String, usize),
    GameArgDelete(usize),
    GameArgShiftUp(usize),
    GameArgShiftDown(usize),
}

#[derive(Debug, Clone)]
pub enum ManageModsMessage {
    ScreenOpen,
    ToggleCheckbox((String, String), bool),
    ToggleCheckboxLocal(String, bool),
    DeleteSelected,
    DeleteFinished(Result<Vec<String>, String>),
    LocalDeleteFinished(Result<(), String>),
    LocalIndexLoaded(HashSet<String>),
    ToggleSelected,
    ToggleFinished(Result<(), String>),
    UpdateMods,
    UpdateModsFinished(Result<(), String>),
    UpdateCheckResult(Option<Vec<(String, String)>>),
    UpdateCheckToggle(usize, bool),
}

#[derive(Debug, Clone)]
pub enum InstallModsMessage {
    SearchResult(Result<(Search, Instant), String>),
    Open,
    SearchInput(String),
    ImageDownloaded(Result<ImageResult, String>),
    Click(usize),
    BackToMainScreen,
    LoadData(Result<Box<ProjectInfo>, String>),
    Download(usize),
    DownloadComplete(Result<String, String>),
    IndexUpdated(Result<ModIndex, String>),
}

#[derive(Debug, Clone)]
pub enum InstallOptifineMessage {
    ScreenOpen,
    SelectInstallerStart,
    SelectInstallerEnd(Option<rfd::FileHandle>),
    End(Result<(), String>),
}

#[derive(Debug, Clone)]
pub enum EditPresetsMessage {
    Open,
    ToggleCheckbox((String, String), bool),
    ToggleCheckboxLocal(String, bool),
    SelectAll,
    BuildYourOwn,
    BuildYourOwnEnd(Result<Vec<u8>, String>),
    Load,
    LoadComplete(Result<(), String>),
    RecommendedModCheck(Result<Vec<RecommendedMod>, String>),
    RecommendedToggle(usize, bool),
    RecommendedDownload,
    RecommendedDownloadEnd(Result<(), String>),
}

#[derive(Debug, Clone)]
pub enum Message {
    CreateInstance(CreateInstanceMessage),
    EditInstance(EditInstanceMessage),
    ManageMods(ManageModsMessage),
    InstallMods(InstallModsMessage),
    InstallOptifine(InstallOptifineMessage),
    InstallFabric(InstallFabricMessage),
    EditPresets(EditPresetsMessage),
    CoreOpenDir(String),
    LaunchInstanceSelected(String),
    LaunchUsernameSet(String),
    LaunchStart,
    LaunchScreenOpen {
        message: Option<String>,
        clear_selection: bool,
    },
    LaunchEnd(GameLaunchResult),
    LaunchKill,
    LaunchKillEnd(Result<(), String>),
    DeleteInstanceMenu,
    DeleteInstance,
    InstallForgeStart,
    InstallForgeEnd(Result<(), String>),
    InstallPaperStart,
    InstallPaperEnd(Result<(), String>),
    UninstallLoaderConfirm(Box<Message>, String),
    UninstallLoaderFabricStart,
    UninstallLoaderForgeStart,
    UninstallLoaderOptiFineStart,
    UninstallLoaderPaperStart,
    UninstallLoaderEnd(Result<Loader, String>),
    CoreErrorCopy,
    CoreTick,
    CoreTickConfigSaved(Result<(), String>),
    CoreListLoaded(Result<(Vec<String>, bool), String>),
    CoreCopyText(String),
    CoreOpenChangeLog,
    CoreEvent(iced::Event, iced::event::Status),
    LaunchEndedLog(Result<(ExitStatus, String), String>),
    LaunchCopyLog,
    UpdateCheckResult(Result<UpdateCheckInfo, String>),
    UpdateDownloadStart,
    UpdateDownloadEnd(Result<(), String>),
    ManageModsSelectAll,
    LauncherSettingsThemePicked(String),
    LauncherSettingsStylePicked(String),
    LauncherSettingsOpen,
    ServerManageOpen {
        selected_server: Option<String>,
        message: Option<String>,
    },
    ServerManageSelectedServer(String),
    ServerManageStartServer(String),
    ServerManageStartServerFinish(Result<(Arc<Mutex<Child>>, bool), String>),
    ServerManageEndedLog(Result<(ExitStatus, String), String>),
    ServerManageKillServer(String),
    ServerManageEditCommand(String, String),
    ServerManageCopyLog,
    ServerManageSubmitCommand(String),
    ServerCreateScreenOpen,
    ServerCreateVersionsLoaded(Result<Vec<ListEntry>, String>),
    ServerCreateNameInput(String),
    ServerCreateVersionSelected(ListEntry),
    ServerCreateStart,
    ServerCreateEnd(Result<String, String>),
    ServerDeleteOpen,
    ServerDeleteConfirm,
    ServerEditModsOpen,
}

/// The home screen of the launcher.
#[derive(Default)]
pub struct MenuLaunch {
    pub message: String,
    pub java_recv: Option<Receiver<GenericProgress>>,
    pub asset_recv: Option<Receiver<GenericProgress>>,
}

impl MenuLaunch {
    pub fn with_message(message: String) -> Self {
        Self {
            message,
            java_recv: None,
            asset_recv: None,
        }
    }
}

/// The screen where you can edit an instance/server.
pub struct MenuEditInstance {
    pub config: InstanceConfigJson,
    pub slider_value: f32,
    pub slider_text: String,
}

pub enum SelectedState {
    All,
    Some,
    None,
}

#[derive(Debug, Clone)]
pub enum ModListEntry {
    Downloaded { id: String, config: Box<ModConfig> },
    Local { file_name: String },
}

impl ModListEntry {
    pub fn is_manually_installed(&self) -> bool {
        match self {
            ModListEntry::Local { .. } => true,
            ModListEntry::Downloaded { config, .. } => config.manually_installed,
        }
    }

    pub fn name(&self) -> String {
        match self {
            ModListEntry::Local { file_name } => file_name.clone(),
            ModListEntry::Downloaded { config, .. } => config.name.clone(),
        }
    }

    pub fn id(&self) -> SelectedMod {
        match self {
            ModListEntry::Local { file_name } => SelectedMod::Local {
                file_name: file_name.clone(),
            },
            ModListEntry::Downloaded { id, config } => SelectedMod::Downloaded {
                name: config.name.clone(),
                id: id.clone(),
            },
        }
    }
}

pub struct MenuEditMods {
    pub config: InstanceConfigJson,
    pub mods: ModIndex,
    pub locally_installed_mods: HashSet<String>,
    pub selected_mods: HashSet<SelectedMod>,
    pub sorted_mods_list: Vec<ModListEntry>,
    pub selected_state: SelectedState,
    pub available_updates: Vec<(String, String, bool)>,
    pub mod_update_progress: Option<ProgressBar<GenericProgress>>,
}

impl MenuEditMods {
    pub fn update_locally_installed_mods(
        idx: &ModIndex,
        selected_instance: InstanceSelection,
        dir: &Path,
    ) -> Command<Message> {
        let mut blacklist = Vec::new();
        for mod_info in idx.mods.values() {
            for file in &mod_info.files {
                blacklist.push(file.filename.clone());
            }
        }
        Command::perform(
            get_locally_installed_mods(selected_instance.get_dot_minecraft_path(dir), blacklist),
            |n| Message::ManageMods(ManageModsMessage::LocalIndexLoaded(n)),
        )
    }
}

pub enum MenuCreateInstance {
    Loading {
        progress_receiver: Receiver<()>,
        progress_number: f32,
    },
    Loaded {
        instance_name: String,
        selected_version: Option<ListEntry>,
        progress: Option<ProgressBar<DownloadProgress>>,
        download_assets: bool,
        combo_state: Box<iced::widget::combo_box::State<ListEntry>>,
    },
}

pub enum MenuInstallFabric {
    Loading(bool),
    Loaded {
        is_quilt: bool,
        fabric_version: Option<String>,
        fabric_versions: Vec<String>,
        progress: Option<ProgressBar<GenericProgress>>,
    },
    Unsupported(bool),
}

impl MenuInstallFabric {
    pub fn is_quilt(&self) -> bool {
        match self {
            MenuInstallFabric::Loading(is_quilt)
            | MenuInstallFabric::Loaded { is_quilt, .. }
            | MenuInstallFabric::Unsupported(is_quilt) => *is_quilt,
        }
    }
}

pub struct MenuInstallForge {
    pub forge_progress: ProgressBar<ForgeInstallProgress>,
    pub java_progress: ProgressBar<GenericProgress>,
    pub is_java_getting_installed: bool,
}

pub struct MenuLauncherUpdate {
    pub url: String,
    pub progress: Option<ProgressBar<GenericProgress>>,
}

pub struct MenuModsDownload {
    pub query: String,
    pub results: Option<Search>,
    pub result_data: HashMap<String, ProjectInfo>,
    pub config: InstanceConfigJson,
    pub json: VersionDetails,
    pub opened_mod: Option<usize>,
    pub latest_load: Instant,
    pub is_loading_search: bool,
    pub mods_download_in_progress: HashSet<String>,
    pub mod_index: ModIndex,
}

pub struct MenuLauncherSettings;

pub struct MenuEditPresets {
    pub inner: MenuEditPresetsInner,
    pub progress: Option<ProgressBar<GenericProgress>>,
}

pub enum MenuEditPresetsInner {
    Build {
        mods: Vec<ModListEntry>,
        selected_mods: HashSet<SelectedMod>,
        selected_state: SelectedState,
        is_building: bool,
    },
    Recommended {
        mods: Option<Vec<(bool, RecommendedMod)>>,
        error: Option<String>,
        progress: ProgressBar<GenericProgress>,
    },
}

/// The enum that represents which menu is opened currently.
pub enum State {
    Welcome,
    Launch(MenuLaunch),
    ChangeLog,
    EditInstance(MenuEditInstance),
    EditMods(MenuEditMods),
    Create(MenuCreateInstance),
    Error {
        error: String,
    },
    ConfirmAction {
        msg1: String,
        msg2: String,
        yes: Message,
        no: Message,
    },
    InstallPaper,
    InstallFabric(MenuInstallFabric),
    InstallForge(MenuInstallForge),
    InstallOptifine(MenuInstallOptifine),
    InstallJava(ProgressBar<GenericProgress>),
    RedownloadAssets {
        progress: ProgressBar<GenericProgress>,
        java_recv: Option<Receiver<GenericProgress>>,
    },
    UpdateFound(MenuLauncherUpdate),
    ModsDownload(Box<MenuModsDownload>),
    LauncherSettings,
    ServerManage(MenuServerManage),
    ServerCreate(MenuServerCreate),
    ManagePresets(MenuEditPresets),
}

pub struct MenuServerManage {
    pub java_install_recv: Option<Receiver<GenericProgress>>,
    pub message: Option<String>,
}

pub enum MenuServerCreate {
    LoadingList {
        progress_receiver: Receiver<()>,
        progress_number: f32,
    },
    Loaded {
        name: String,
        versions: iced::widget::combo_box::State<ListEntry>,
        selected_version: Option<ListEntry>,
    },
    Downloading {
        progress: ProgressBar<GenericProgress>,
    },
}

#[derive(Default)]
pub struct MenuInstallOptifine {
    pub optifine_install_progress: Option<ProgressBar<OptifineInstallProgress>>,
    pub java_install_progress: Option<ProgressBar<GenericProgress>>,
    pub is_java_being_installed: bool,
}

pub struct InstanceLog {
    pub log: String,
    pub has_crashed: bool,
    pub command: String,
}

pub struct Launcher {
    pub state: State,
    pub dir: PathBuf,
    pub selected_instance: Option<InstanceSelection>,
    pub client_version_list_cache: Option<Vec<ListEntry>>,
    pub server_version_list_cache: Option<Vec<ListEntry>>,
    pub client_list: Option<Vec<String>>,
    pub server_list: Option<Vec<String>>,
    pub config: Option<LauncherConfig>,
    pub client_processes: HashMap<String, ClientProcess>,
    pub server_processes: HashMap<String, ServerProcess>,
    pub client_logs: HashMap<String, InstanceLog>,
    pub server_logs: HashMap<String, InstanceLog>,
    pub images_bitmap: HashMap<String, Handle>,
    pub images_svg: HashMap<String, iced::widget::svg::Handle>,
    pub images_downloads_in_progress: HashSet<String>,
    pub images_to_load: Mutex<HashSet<String>>,
    pub theme: LauncherTheme,
    pub style: Arc<Mutex<LauncherStyle>>,
    pub window_size: (u32, u32),
}

pub struct ClientProcess {
    pub child: Arc<Mutex<Child>>,
    pub receiver: Option<Receiver<LogLine>>,
}

pub struct ServerProcess {
    pub child: Arc<Mutex<Child>>,
    pub receiver: Option<Receiver<String>>,
    pub stdin: Option<ChildStdin>,
    pub is_classic_server: bool,
    pub name: String,
    pub has_issued_stop_command: bool,
}

impl Drop for ServerProcess {
    fn drop(&mut self) {
        if !self.has_issued_stop_command {
            info!("Force-Killing server {}\n       You should be a bit more careful before closing the launcher window", self.name);
            let mut lock = self.child.lock().unwrap();
            let _ = lock.start_kill();
        }
    }
}

impl Launcher {
    pub fn load_new(message: Option<String>, is_new_user: bool) -> Result<Self, JsonFileError> {
        let launcher_dir = get_launcher_dir();

        let (mut config, theme, style) = load_config_and_theme(&launcher_dir)?;

        let launch = State::Launch(if let Some(message) = message {
            MenuLaunch::with_message(message)
        } else {
            MenuLaunch::default()
        });

        let state = if is_new_user {
            State::Welcome
        } else if let Some(config) = &mut config {
            let version = config.version.as_deref().unwrap_or("0.3.0");
            if version == LAUNCHER_VERSION_NAME {
                launch
            } else {
                config.version = Some(LAUNCHER_VERSION_NAME.to_owned());
                State::ChangeLog
            }
        } else {
            launch
        };
        *STYLE.lock().unwrap() = style;

        Ok(Self {
            dir: launcher_dir,
            client_list: None,
            server_list: None,
            state,
            client_processes: HashMap::new(),
            config,
            client_logs: HashMap::new(),
            selected_instance: None,
            images_bitmap: HashMap::new(),
            images_svg: HashMap::new(),
            images_downloads_in_progress: HashSet::new(),
            images_to_load: Mutex::new(HashSet::new()),
            theme,
            style: STYLE.clone(),
            client_version_list_cache: None,
            server_version_list_cache: None,
            server_processes: HashMap::new(),
            server_logs: HashMap::new(),
            window_size: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        })
    }

    pub fn with_error(error: impl std::fmt::Display) -> Self {
        let launcher_dir = get_launcher_dir();

        let (config, theme, style) = load_config_and_theme(&launcher_dir).unwrap_or((
            None,
            LauncherTheme::default(),
            LauncherStyle::default(),
        ));
        *STYLE.lock().unwrap() = style;

        Self {
            dir: launcher_dir,
            state: State::Error {
                error: format!("Error: {error}"),
            },
            client_list: None,
            server_list: None,
            config,
            client_processes: HashMap::new(),
            client_logs: HashMap::new(),
            selected_instance: None,
            images_bitmap: HashMap::new(),
            images_svg: HashMap::new(),
            images_downloads_in_progress: HashSet::new(),
            images_to_load: Mutex::new(HashSet::new()),
            theme,
            style: STYLE.clone(),
            client_version_list_cache: None,
            server_processes: HashMap::new(),
            server_logs: HashMap::new(),
            server_version_list_cache: None,
            window_size: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        }
    }

    pub fn set_error(&mut self, error: impl ToString) {
        self.state = State::Error {
            error: error.to_string(),
        }
    }

    pub fn go_to_launch_screen(&mut self, message: Option<String>) -> Command<Message> {
        self.state = State::Launch(match message {
            Some(message) => MenuLaunch::with_message(message),
            None => MenuLaunch::default(),
        });
        Command::perform(
            get_entries("instances".to_owned(), false),
            Message::CoreListLoaded,
        )
    }

    pub fn edit_instance_w(&mut self) {
        let selected_instance = self.selected_instance.clone().unwrap();
        match self.edit_instance(&selected_instance) {
            Ok(()) => {}
            Err(err) => self.set_error(err),
        }
    }
}

fn get_launcher_dir() -> PathBuf {
    match file_utils::get_launcher_dir_s() {
        Ok(dir) => dir,
        Err(err) => {
            err!("Could not load launcher dir! This is a bug! Please report!");
            pt!("{err}\n");

            ERROR_STARTING_LAUNCHER
                .lock()
                .unwrap()
                .push_str(&err.to_string());

            xdialog::XDialogBuilder::new().run(error_starting_launcher);
            std::process::exit(1);
        }
    }
}

lazy_static! {
    static ref ERROR_STARTING_LAUNCHER: Mutex<String> = Mutex::new(String::new());
}

fn error_starting_launcher() -> i32 {
    let err = ERROR_STARTING_LAUNCHER.lock().unwrap().clone();

    let result = xdialog::show_message(xdialog::XDialogOptions {
        title: "Error starting launcher!".to_owned(),
        main_instruction: "Could not start launcher (error loading launcher directory)!".to_owned(),
        message: format!("This is a bug! Please report (and try starting again)!\n{err}"),
        icon: xdialog::XDialogIcon::Error,
        buttons: vec![
            "Copy".to_owned(),
            "Join Discord".to_owned(),
            "Exit".to_owned(),
        ],
    })
    .unwrap();

    if let xdialog::XDialogResult::ButtonPressed(result) = result {
        if result == 0 {
            arboard::Clipboard::new().unwrap().set_text(&err).unwrap();
            return error_starting_launcher();
        } else if result == 1 {
            let _ = open_file_explorer("https://discord.gg/bWqRaSXar5");
            return error_starting_launcher();
        }
    }

    1
}

fn load_config_and_theme(
    launcher_dir: &Path,
) -> Result<(Option<LauncherConfig>, LauncherTheme, LauncherStyle), JsonFileError> {
    let config = LauncherConfig::load(launcher_dir)?;
    let theme = match config.theme.as_deref() {
        Some("Dark") => LauncherTheme::Dark,
        Some("Light") => LauncherTheme::Light,
        None => LauncherTheme::default(),
        _ => {
            err!("Unknown style: {:?}", config.theme);
            LauncherTheme::default()
        }
    };
    let style = match config.style.as_deref() {
        Some("Brown") => LauncherStyle::Brown,
        Some("Purple") => LauncherStyle::Purple,
        Some("Light Blue") => LauncherStyle::LightBlue,
        None => LauncherStyle::default(),
        _ => {
            err!("Unknown style: {:?}", config.style);
            LauncherStyle::default()
        }
    };
    Ok((Some(config), theme, style))
}

pub async fn get_entries(path: String, is_server: bool) -> Result<(Vec<String>, bool), String> {
    let dir_path = file_utils::get_launcher_dir()
        .await
        .map_err(|n| n.to_string())?
        .join(path);
    if !dir_path.exists() {
        tokio::fs::create_dir_all(&dir_path)
            .await
            .path(&dir_path)
            .map_err(|n| n.to_string())?;
        return Ok((Vec::new(), is_server));
    }

    let mut dir = tokio::fs::read_dir(&dir_path)
        .await
        .path(dir_path)
        .map_err(|n| n.to_string())?;

    let mut subdirectories = Vec::new();

    while let Ok(Some(entry)) = dir.next_entry().await {
        if entry.path().is_dir() {
            if let Some(file_name) = entry.file_name().to_str() {
                subdirectories.push(file_name.to_owned());
            }
        }
    }

    Ok((subdirectories, is_server))
}

pub struct ProgressBar<T: Progress> {
    pub num: f32,
    pub message: Option<String>,
    pub receiver: Receiver<T>,
    pub progress: T,
}

impl<T: Default + Progress> ProgressBar<T> {
    pub fn with_recv(receiver: Receiver<T>) -> Self {
        Self {
            num: 0.0,
            message: None,
            receiver,
            progress: T::default(),
        }
    }

    pub fn with_recv_and_msg(receiver: Receiver<T>, msg: String) -> Self {
        Self {
            num: 0.0,
            message: Some(msg),
            receiver,
            progress: T::default(),
        }
    }
}

impl<T: Progress> ProgressBar<T> {
    pub fn tick(&mut self) -> bool {
        let mut has_ticked = false;
        while let Ok(progress) = self.receiver.try_recv() {
            self.num = progress.get_num();
            self.message = progress.get_message();
            self.progress = progress;
            has_ticked = true;
        }
        has_ticked
    }
}

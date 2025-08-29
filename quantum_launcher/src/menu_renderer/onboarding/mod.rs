use std::sync::LazyLock;

use iced::widget::image::Handle;

mod legacy;
mod v0_4_2;
mod welcome;

pub use v0_4_2::changelog;

pub static IMG_LAUNCHER: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(include_bytes!("../../../../quantum_launcher.png").as_slice())
});
pub static IMG_NEW: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(include_bytes!("../../../../assets/screenshots/new.png").as_slice())
});
pub static IMG_LOADERS: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(
        include_bytes!("../../../../assets/screenshots/install_loader.png").as_slice(),
    )
});
pub static IMG_MOD_STORE: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(include_bytes!("../../../../assets/screenshots/mod_store.png").as_slice())
});
pub static IMG_OLD_MC: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(include_bytes!("../../../../assets/screenshots/old_mc.png").as_slice())
});
pub static IMG_THEMES: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(include_bytes!("../../../../assets/screenshots/themes.png").as_slice())
});
pub static IMG_PRESETS: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(include_bytes!("../../../../assets/screenshots/presets.png").as_slice())
});
pub static IMG_LOGO: LazyLock<Handle> = LazyLock::new(|| {
    Handle::from_bytes(include_bytes!("../../../../assets/icon/ql_logo.png").as_slice())
});

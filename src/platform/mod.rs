#![warn(missing_docs)]

use std::path::Path;

#[cfg(any(doc, target_os = "macos"))]
pub mod macos;

#[cfg(any(doc, target_os = "windows"))]
pub mod windows;

#[cfg(any(doc, target_os = "linux"))]
pub mod linux;

/// Opens the settings file
pub fn open_settings() {
    #[cfg(target_os = "macos")]
    macos::open_settings()
}

/// Gets an iced image handle.
/// 
/// On macos, if the path ends with `.icns`, it parses it as a `.icns` file.
/// 
/// In all other cases, it tries to get an iced image handle with 
/// [`iced::widget::image::Handle::from_path`].
pub fn get_img_handle(path: &Path) -> Option<iced::widget::image::Handle> {
    if !path.exists() {
        return None;
    }


    #[cfg(target_os = "macos")]
    if let Some(ext) = path.extension().unwrap() && ext == "icns" {
        return macos::handle_from_icns(path);
    }

    Some(iced::widget::image::Handle::from_path(path))
}

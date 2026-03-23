//! This has all the logic regarding the cliboard history
use arboard::ImageData;

use crate::{
    app::apps::{AppCommand, SimpleApp},
    commands::Function,
};

/// The kinds of clipboard content that rustcast can handle and their contents
#[derive(Debug, Clone)]
pub enum ClipBoardContentType {
    Text(String),
    Image(ImageData<'static>),
}

impl ClipBoardContentType {
    /// Returns the iced element for rendering the clipboard item, and the entire content since the
    /// display name is only the first line
    pub fn to_app(&self) -> SimpleApp {
        let mut name = match self {
            ClipBoardContentType::Image(_) => "<img>".to_string(),
            ClipBoardContentType::Text(a) => a.to_owned(),
        };

        let self_clone = self.clone();
        let name_lc = name.clone();

        // only get the first line from the contents
        name = name.lines().next().unwrap_or("").to_string();

        SimpleApp::new_builtin(
            &name,
            &name_lc,
            "Clipboard Item",
            AppCommand::Function(Function::CopyToClipboard(self_clone.clone())),
        )
    }
}

impl PartialEq for ClipBoardContentType {
    /// Let clipboard items be comparable
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Image(a), Self::Image(b)) => a.bytes == b.bytes,
            _ => false
        }
    }
}

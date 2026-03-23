//! This has all the logic regarding the cliboard history
use arboard::ImageData;
use iced::widget::{Button, Text, text::Wrapping};

use crate::{
    app::{Message, apps::{AppCommand, SimpleApp}},
    commands::Function,
};

/// The kinds of clipboard content that rustcast can handle and their contents
#[derive(Debug, Clone)]
pub enum ClipboardContent {
    Text(String),
    Image(ImageData<'static>),
}

impl ClipboardContent {
    /// Returns the iced element for rendering the clipboard item, and the entire content since the
    /// display name is only the first line
    pub fn render<'a>(&'a self, theme: &crate::config::Theme) -> iced::Element<'a, Message> {
        let mut text = match self {
            ClipboardContent::Image(_) => "<img>".to_string(),
            ClipboardContent::Text(a) => a.to_owned(),
        };

        // only get the first line from the contents
        text = text.lines().next().unwrap_or("").to_string();

        let text = Text::new(text)
            .font(theme.font())
            .size(16)
            .wrapping(Wrapping::None)
            .color(theme.text_color(1.0));

        Button::new(text)
            .on_press(Message::CopyToClipboard(self.clone())) // shhhhhh
            .into()
    }
}

impl PartialEq for ClipboardContent {
    /// Let clipboard items be comparable
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Image(a), Self::Image(b)) => a.bytes == b.bytes,
            _ => false
        }
    }
}

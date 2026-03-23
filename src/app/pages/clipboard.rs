use std::collections::VecDeque;
use std::fmt::Debug;

use iced::widget::{
    Scrollable, scrollable,
    scrollable::{Direction, Scrollbar},
};

use crate::{app::pages::prelude::*, functions::clipboard::ClipboardContent};

pub struct ClipboardState {
    clipboard: arboard::Clipboard,
    content: VecDeque<ClipboardContent>
}

impl Debug for ClipboardState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClipboardState {{ content: {:?} }}", self.content)
    }
}

impl ClipboardState {
    /// Inits the state.
    /// 
    /// # Errors
    /// 
    /// If there was an error initing the underlying [`arboard::Clipboard`].
    pub fn new() -> Result<Self, arboard::Error> {
        Ok(Self {
            clipboard: arboard::Clipboard::new()?,
            content: VecDeque::default()
        })
    }

    /// Adds an item to the clipboard.
    /// 
    /// The return value is `true` if no item was added (cap reached), and `false` if it wasn't.
    pub fn add_item(&mut self, item: ClipboardContent) -> bool {
        tracing::trace!(target: "clipboard_page", "Adding item {item:?} to clipboard {self:?}");

        if self.content.len() < 50 {
            self.content.push_front(item);
            true
        }
        else {
            false
        }
    }

    /// Gets the number of items stored in the clipboard
    pub fn len(&self) -> usize {
        self.content.len()
    }

    
    /// Writes the item to the device clipboard
    pub fn write_to_clipboard(&mut self, item: &ClipboardContent) -> Result<(), arboard::Error> {
        match item {
            ClipboardContent::Text(text) => self.clipboard.set_text(text),
            ClipboardContent::Image(image) => self.clipboard.set_image(image.clone()) // Think the clone is needed
        }
    }
}

pub fn render<'a>(
    state: &'a ClipboardState,
    focussed_id: u32,
    theme: &'a Theme
) -> Element<'a, Message> {
    let theme_clone = theme.clone();
    let theme_clone_2 = theme.clone();
    container(Row::from_vec(vec![
        container(
            scrollable(
                state.content
                    .iter()
                    .enumerate()
                    .map(|(i, content)| {
                        tracing::trace!(target: "render", "Drawing clipboard item index {i}: {content:?}");

                        // I'd be surprised if you get 4 billion entries
                        #[allow(clippy::cast_possible_truncation)]
                        content.render(theme)
                    })
                    .collect::<Column<_>>()
                    .width(WINDOW_WIDTH / 3.),
            )
            .id("results"),
        )
        .height(385)
        .style(move |_| result_row_container_style(&theme_clone_2, false))
        .into(),
        container(Scrollable::with_direction(
            Text::new(
                state.content
                    .get(focussed_id as usize)
                    .map(|x| if let ClipboardContent::Text(text) = x { text } else { "img" })
                    .unwrap_or_default(),
            )
            .height(385)
            .width(Length::Fill)
            .align_x(Alignment::Start)
            .font(theme.font())
            .size(16),
            Direction::Both {
                vertical: Scrollbar::new().scroller_width(0.).width(0.),
                horizontal: Scrollbar::new().scroller_width(0.).width(0.),
            },
        ))
        .padding(10)
        .style(move |_| result_row_container_style(&theme_clone, false))
        .width((WINDOW_WIDTH / 3.) * 2.)
        .into(),
    ]))
    .height(280)
    .into()
}

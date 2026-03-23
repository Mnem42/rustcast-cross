use std::collections::VecDeque;

use iced::widget::{
    Scrollable, scrollable,
    scrollable::{Direction, Scrollbar},
};

use crate::{app::pages::prelude::*, functions::clipboard::ClipboardContent};

#[derive(Debug, Default)]
pub struct ClipboardState {
    content: VecDeque<ClipboardContent>
}

impl ClipboardState {
    /// Adds an item to the clipboard.
    /// 
    /// The return value is `true` if no item was added (cap reached), and `false` if it wasn't.
    pub fn add_item(&mut self, item: ClipboardContent) -> bool {
        if self.content.len() < 50 {
            self.content.push_back(item);
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
}

pub fn clipboard_view(
    state: &ClipboardState,
    focussed_id: u32,
    theme: &Theme,
    focus_id: u32,
) -> Element<'static, Message> {
    let theme_clone = theme.clone();
    let theme_clone_2 = theme.clone();
    container(Row::from_vec(vec![
        container(
            scrollable(
                state.content
                    .iter()
                    .enumerate()
                    .map(|(i, content)| {
                        // I'd be surprised if you get 4 billion entries
                        #[allow(clippy::cast_possible_truncation)]
                        content.to_app().render(theme.clone(), i as u32, focus_id)
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
                    .map(|x| x.to_app().alias)
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

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
};

use crate::{theme::theme, views::ViewConfig};

pub struct ViewTabs<'a> {
    views: &'a [ViewConfig],
    active_index: usize,
    loading: bool,
}

impl<'a> ViewTabs<'a> {
    pub fn new(views: &'a [ViewConfig], active_index: usize, loading: bool) -> Self {
        Self { views, active_index, loading }
    }
}

impl Widget for ViewTabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut spans = Vec::new();

        for (i, view) in self.views.iter().enumerate() {
            let is_active = i == self.active_index;
            let suffix = if is_active && self.loading { "…" } else { "" };
            let label = format!(" [{}] {}{} ", view.key, view.name, suffix);

            let style = if is_active {
                Style::default()
                    .fg(theme().table_border.fg.unwrap_or(ratatui::style::Color::Reset))
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
            } else {
                Style::default().fg(ratatui::style::Color::DarkGray)
            };

            spans.push(Span::styled(label, style));
            spans.push(Span::raw(" "));
        }

        Line::from(spans).render(area, buf);
    }
}

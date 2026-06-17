mod focusable;
mod image_list;
mod project_list;

use ratatui::widgets::ListState;

use crate::tui::App;
use crate::tui::prelude::*;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let h_layout = Layout::default()
            .direction(Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);
        let v_layout = Layout::default()
            .direction(Vertical)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(h_layout[1]);

        StatefulWidget::render(
            self.project_list(),
            h_layout[0],
            buf,
            &mut ListState::default().with_selected(Some(self.selected_project)),
        );

        StatefulWidget::render(
            self.image_list(),
            v_layout[0],
            buf,
            &mut ListState::default().with_selected(Some(self.selected_image)),
        );
    }
}

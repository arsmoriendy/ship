mod focusable;
mod image_list;
mod project_list;

use crate::tui::prelude::*;
use crate::tui::state::AppState;
use crate::tui::widgets::image_list::ImageList;
use crate::tui::widgets::project_list::ProjectList;

pub struct RootWidget {}
impl StatefulWidget for RootWidget {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let h_layout = Layout::default()
            .direction(Horizontal)
            .constraints(vec![Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(area);

        StatefulWidget::render(ProjectList {}, h_layout[0], buf, state);
        StatefulWidget::render(ImageList {}, h_layout[1], buf, state);
    }
}

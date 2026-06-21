mod focusable;
mod footer;
mod image_list;
mod project_list;

use crate::tui::prelude::*;
use crate::tui::state::AppState;
use crate::tui::widgets::footer::Footer;
use crate::tui::widgets::image_list::ImageList;
use crate::tui::widgets::project_list::ProjectList;

pub struct RootWidget {}
impl StatefulWidget for RootWidget {
    type State = AppState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let [main, footer] = vertical![*=1, ==1].areas(area);
        let [projects, images] = horizontal![==50%,==50%].areas(main);

        StatefulWidget::render(ProjectList {}, projects, buf, state);
        StatefulWidget::render(ImageList {}, images, buf, state);
        StatefulWidget::render(Footer {}, footer, buf, state);
    }
}

macro_rules! cmd {
    ($cmd:expr, $($args:expr),+) => {
        Command::new($cmd)
            .args([$($args),+])
    };
}

macro_rules! add_app_action {
    ($name:ident,  $state:ident, $block:block) => {
        use crate::tui::{App, prelude::*};
        impl App {
            pub async fn $name(&self) -> Result<()> {
                let $state = &self.state;
                $block
            }
        }
    };
    ($self:ident; $name:ident, $block:block) => {
        use crate::tui::{App, prelude::*};
        impl App {
            pub async fn $name(&self) -> Result<()> {
                let $self = self;
                $block
            }
        }
    };
    ($name:ident, $state:ident, $terminal:ident, $block:block) => {
        use crate::tui::{App, prelude::*};
        impl App {
            pub async fn $name(&self, terminal: &mut DefaultTerminal) -> Result<()> {
                let $state = &self.state;
                let $terminal = terminal;
                $block
            }
        }
    };
    ($self:ident; $name:ident, $terminal:ident, $block:block) => {
        use crate::tui::{App, prelude::*};
        impl App {
            pub async fn $name(&self, terminal: &mut DefaultTerminal) -> Result<()> {
                let $self = self;
                let $terminal = terminal;
                $block
            }
        }
    };
}

macro_rules! handle_spawn_error {
    ($state:expr, $handle:expr) => {
        tokio::spawn(async move {
            use crate::tui::state::Focus;
            if let Err(err) = $handle.await.unwrap() {
                let mut mtx = $state.lock().await;
                mtx.focus = Focus::Popup(Box::new(mtx.focus.clone()));
                mtx.popup = Some(ErrorPopup(err.to_string()))
            }
        });
    };
}

macro_rules! docker {
    ($($args:expr),+) => {
        Command::new("docker")
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
    ($name:ident,  $state:ident, $terminal:ident, $block:block) => {
        use crate::tui::{App, prelude::*};
        impl App {
            pub async fn $name(&self, terminal: &mut DefaultTerminal) -> Result<()> {
                let $state = &self.state;
                let $terminal = terminal;
                $block
            }
        }
    };
}

macro_rules! handle_spawn_error {
    ($state:expr, $handle:expr) => {
        tokio::spawn(async move {
            if let Err(err) = $handle.await.unwrap() {
                let mut mtx = $state.lock().await;
                mtx.popup = Some(ErrorPopup(err.to_string()))
            }
        });
    };
}

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

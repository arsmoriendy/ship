use crate::prelude::*;
use crate::tui::prelude::*;

pub struct ExternalCommand<'a> {
    terminal: &'a mut DefaultTerminal,
}

impl<'a> ExternalCommand<'a> {
    pub fn init(terminal: &'a mut DefaultTerminal) -> Result<Self> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        terminal.clear()?;
        Ok(ExternalCommand { terminal })
    }
}

impl<'a> Drop for ExternalCommand<'a> {
    fn drop(&mut self) {
        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        self.terminal.clear().unwrap();
    }
}

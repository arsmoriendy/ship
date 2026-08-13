use super::prelude::*;
use std::io::BufRead;

macro_rules! sh {
    ($cmd:expr) => {
        Command::new("sh").args(["-c", $cmd])
    };
}

pub struct ExternalCommand<'a> {
    terminal: &'a mut DefaultTerminal,
}

impl<'a> Drop for ExternalCommand<'a> {
    fn drop(&mut self) {
        use crossterm::style::Stylize as CTStylize;

        println!("{}", CTStylize::yellow("[ship] Press enter to continue"));
        stdin().lock().read_line(&mut String::new()).unwrap();

        stdout().execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        self.terminal.clear().unwrap();
    }
}

impl<'a> ExternalCommand<'a> {
    pub fn init(terminal: &'a mut DefaultTerminal) -> Result<Self> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        terminal.clear()?;
        Ok(ExternalCommand { terminal })
    }

    pub fn sh(cmd: &'a str) -> Result<String> {
        let res = sh!(cmd).output()?;
        if !res.status.success() {
            return Err(anyhow!("{}", String::from_utf8(res.stderr)?))
                .with_context(|| format!("Unsuccessful command \"{}\"", cmd));
        }
        Ok(String::from_utf8(res.stdout)?)
    }

    pub fn shout(cmd: &'a str, terminal: &'a mut DefaultTerminal) -> Result<()> {
        let _cmd = Self::init(terminal);
        let status = sh!(cmd).spawn()?.wait()?;
        if !status.success() {
            return Err(anyhow!("Unsuccessful command \"{}\"", cmd));
        }
        Ok(())
    }
}

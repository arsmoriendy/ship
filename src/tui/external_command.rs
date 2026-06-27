use super::prelude::*;
use crate::{
    config::{CommandBehaviour, RegistryCommands},
    project::Project,
};

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
                .with_context(|| format!("failed to run command: \"{}\"", cmd));
        }
        Ok(String::from_utf8(res.stdout)?)
    }

    pub fn shout(cmd: &'a str, terminal: &'a mut DefaultTerminal) -> Result<()> {
        let _cmd = Self::init(terminal);
        let status = sh!(cmd).spawn()?.wait()?;
        if !status.success() {
            return Err(anyhow!("failed to run command: \"{}\"", cmd));
        }
        Ok(())
    }
}

impl super::App {
    pub async fn current_cmds(&self) -> Result<RegistryCommands> {
        let state = self.state.lock().await;
        let project = &state.projects[state.selected_project];
        let reg = state
            .config
            .project_registries
            .get(&project.name)
            .ok_or(anyhow!("Unconfigured project registry"))?;
        let cmds = &state.config.registry_commands;
        Ok(cmds
            .get(reg.as_str())
            .ok_or(anyhow!("Unconfigured registry commands"))?
            .clone())
    }

    pub async fn update_digests(&self) -> Result<()> {
        let mut cmd = self.current_cmds().await?.list_digests;

        // replace fields
        let state = self.state.lock().await;
        let project = state.projects[state.selected_project].clone();
        cmd = cmd.replace("{project}", &project.name);
        drop(state);

        // TODO: handle errors
        let state = self.state.clone();
        tokio::spawn(async move {
            let mut mtx = state.lock().await;
            mtx.loading = Some("Fetching digests...".to_owned());
            drop(mtx);

            let res = ExternalCommand::sh(&cmd).unwrap();
            let prefixed_digests: Vec<String> = serde_json::from_str(res.as_str()).unwrap();
            let mut digests: Vec<[u8; 32]> = vec![];
            for pd in prefixed_digests {
                digests.push(parse_prefixed_sha256(&pd).unwrap());
            }

            let mut mtx = state.lock().await;
            mtx.store
                .sync(|store| {
                    store.project_registry_digests.insert(project.name, digests);
                })
                .unwrap();
            mtx.loading = None;
        });

        Ok(())
    }

    pub async fn delete_image(&self, image: &Image, terminal: &mut DefaultTerminal) -> Result<()> {
        let digest = image.digest.ok_or(anyhow!("Image has no digest"))?;
        let cmd = self
            .current_cmds()
            .await?
            .delete_image
            .replace("{id}", &encode_hex(image.id))
            .replace("{repository}", &image.repository)
            .replace("{digest}", &encode_hex(digest));

        let state = self.state.lock().await;
        let behaviour = state.config.command_behaviours.delete_image.clone();
        drop(state);

        match behaviour {
            CommandBehaviour::Async => {
                let mutex = self.state.clone();
                let mut state = mutex.lock().await;
                state.loading = Some("Deleting image...".to_owned());
                drop(state);

                tokio::task::spawn(async move {
                    // TODO: handle error
                    ExternalCommand::sh(&cmd).unwrap();

                    let mut state = mutex.lock().await;
                    state.loading = None;
                    drop(state);
                });
            }
            CommandBehaviour::Interactive => {
                ExternalCommand::shout(&cmd, terminal)?;
            }
        }

        Ok(())
    }

    pub async fn push_image(
        &self,
        image: &Image,
        reg: &str,
        terminal: &mut DefaultTerminal,
    ) -> Result<()> {
        let project_name = Project::get_project_name(&image.repository)?;
        let project_url = format!("{}/{}", reg, project_name);

        let mtx = self.state.lock().await;
        let behaviour = mtx.config.command_behaviours.push_image.clone();
        drop(mtx);

        let id_str = encode_hex(image.id);
        for tag in &image.tags {
            let tag_url = format!("{}:{}", project_url, tag);
            match behaviour {
                CommandBehaviour::Async => {
                    let mut mtx = self.state.lock().await;
                    mtx.loading = Some("Pushing image".to_owned());
                    drop(mtx);

                    let state = self.state.clone();
                    let id_str = id_str.clone();
                    tokio::spawn(async move {
                        // TODO: handle errors
                        docker!("image", "tag", &id_str, &tag_url).output().unwrap();
                        docker!("push", &tag_url).output().unwrap();

                        let mut mtx = state.lock().await;
                        mtx.loading = None;
                    });
                }
                CommandBehaviour::Interactive => {
                    let _cmd = ExternalCommand::init(terminal);
                    docker!("image", "tag", &id_str, &tag_url).spawn()?.wait()?;
                    docker!("push", &tag_url).spawn()?.wait()?;
                }
            }
        }

        Ok(())
    }
}

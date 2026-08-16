use crate::{
    image::{Image, LocalImage},
    project::Project,
    tui::{config::CommandBehaviour, external_command::ExternalCommand},
};

add_app_action!(slf; delete_image, terminal, {
    let mtx = slf.state.lock().await;
    let image = mtx.selected_image().clone();
    let behaviour = mtx.config.command_behaviours.delete_image.clone();
    drop(mtx);

    let Image::Local(image)=image else {return Err(anyhow!("Image is not local"))};

    match behaviour {
        CommandBehaviour::Async => slf._delete_image(&image, None).await?,
        CommandBehaviour::Interactive => slf._delete_image(&image, Some(terminal)).await?
    }

    Ok(())
});

impl App {
    pub async fn _delete_image(
        &self,
        image: &LocalImage,
        terminal: Option<&mut DefaultTerminal>,
    ) -> Result<()> {
        let project_name = Project::get_project_name(&image.repository)?.to_owned();
        let digest = image.digest.ok_or(anyhow!("Image has no digest"))?;

        let mtx = self.state.lock().await;
        let cmd = mtx
            .selected_cmds()
            .ok_or(anyhow!("Unconfigured registry or registry commands"))?
            .delete_image
            .replace("{id}", &encode_hex(image.id))
            .replace("{repository}", &image.repository)
            .replace("{digest}", &encode_hex(digest));
        drop(mtx);

        match terminal {
            None => {
                let mut mtx = self.state.lock().await;
                mtx.loading = Some("Deleting image...".to_string());
                drop(mtx);

                let st = self.state.clone();
                let image = image.clone();
                let handle: JoinHandle<Result<()>> = tokio::task::spawn(async move {
                    ExternalCommand::sh(&cmd)?;

                    let mut mtx = st.lock().await;
                    mtx.loading = None;
                    mtx.store.remove_remote_image(
                        &project_name,
                        &(&image)
                            .try_into()
                            .with_context(|| format!("From image: {image:?}"))?,
                    )?;
                    drop(mtx);

                    Ok(())
                });

                let st = self.state.clone();
                handle_spawn_error!(st, handle);
            }
            Some(terminal) => {
                ExternalCommand::shout(&cmd, terminal)?;

                let mut mtx = self.state.lock().await;
                mtx.store
                    .remove_remote_image(&project_name, &image.try_into()?)?;
            }
        }

        Ok(())
    }
}

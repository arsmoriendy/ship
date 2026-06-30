use crate::{config::CommandBehaviour, tui::external_command::ExternalCommand};

add_app_action!(delete_image, state, terminal, {
    let mtx = state.lock().await;
    let project_name = mtx.selected_project().name.clone();
    let image = mtx.selected_image();
    let digest = image.digest.ok_or(anyhow!("Image has no digest"))?;
    let cmd = mtx
        .selected_cmds()
        .ok_or(anyhow!("Unconfigured registry or registry commands"))?
        .delete_image
        .replace("{id}", &encode_hex(image.id))
        .replace("{repository}", &image.repository)
        .replace("{digest}", &encode_hex(digest));
    let behaviour = mtx.config.command_behaviours.delete_image.clone();
    drop(mtx);

    let mut mtx = state.lock().await;
    match behaviour {
        CommandBehaviour::Async => {
            mtx.loading = Some("Deleting image...".to_owned());
            drop(mtx);

            let state = state.clone();
            tokio::task::spawn(async move {
                // TODO: handle error
                ExternalCommand::sh(&cmd).unwrap();

                let mut mtx = state.lock().await;
                mtx.loading = None;
                mtx.store.remove_digest(&project_name, &digest).unwrap();
                drop(mtx);
            });
        }
        CommandBehaviour::Interactive => {
            ExternalCommand::shout(&cmd, terminal)?;
            mtx.store.remove_digest(&project_name, &digest)?;
        }
    }

    Ok(())
});

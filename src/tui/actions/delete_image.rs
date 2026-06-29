use crate::{config::CommandBehaviour, tui::external_command::ExternalCommand};

add_app_action!(delete_image, state, terminal, {
    let mtx = state.lock().await;
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

    match behaviour {
        CommandBehaviour::Async => {
            let mtx = state.clone();
            let mut state = mtx.lock().await;
            state.loading = Some("Deleting image...".to_owned());
            drop(state);

            tokio::task::spawn(async move {
                // TODO: handle error
                ExternalCommand::sh(&cmd).unwrap();

                let mut state = mtx.lock().await;
                state.loading = None;
                drop(state);
            });
        }
        CommandBehaviour::Interactive => {
            ExternalCommand::shout(&cmd, terminal)?;
        }
    }

    Ok(())
});

use crate::{config::CommandBehaviour, tui::external_command::ExternalCommand};

add_app_action!(push_image, state, terminal, {
    let mtx = state.lock().await;

    let project = mtx.selected_project();
    let image = mtx.selected_image();
    let reg = mtx
        .selected_registry()
        .ok_or(anyhow!("Selected project has no configured registry"))?;

    let image_id = encode_hex(image.id);
    let image_tags = image.tags.clone();
    let project_url = format!("{}/{}", reg, project.name);

    let behaviour = mtx.config.command_behaviours.push_image.clone();

    drop(mtx);

    for tag in &image_tags {
        let tag_url = format!("{}:{}", project_url, tag);
        match behaviour {
            CommandBehaviour::Async => {
                let mut mtx = state.lock().await;
                mtx.loading = Some("Pushing image".to_owned());
                drop(mtx);

                let state = state.clone();
                let id_str = image_id.clone();
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
                docker!("image", "tag", &image_id, &tag_url)
                    .spawn()?
                    .wait()?;
                docker!("push", &tag_url).spawn()?.wait()?;
            }
        }
    }

    Ok(())
});

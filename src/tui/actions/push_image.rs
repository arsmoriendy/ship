use crate::{
    image::RemoteImage,
    tui::{config::CommandBehaviour, external_command::ExternalCommand},
};

add_app_action!(push_image, state, terminal, {
    let mtx = state.lock().await;

    let project = mtx.selected_project();
    let project_name = project.name.clone();
    let image = mtx.selected_image().clone();
    let image_id = encode_hex(image.id);
    let reg = mtx
        .selected_registry()
        .ok_or(anyhow!("Selected project has no configured registry"))?;
    let project_url = format!("{}/{}", reg, project.name);

    let behaviour = mtx.config.command_behaviours.push_image.clone();

    drop(mtx);

    for tag in &image.tags {
        let tag_url = format!("{}:{}", project_url, tag);
        let mut mtx = state.lock().await;
        match behaviour {
            CommandBehaviour::Async => {
                mtx.loading = Some("Pushing image".to_owned());
                drop(mtx);

                let st = state.clone();
                let id_str = image_id.clone();
                let project_name = project_name.clone();
                let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
                    docker!("image", "tag", &id_str, &tag_url).output()?;
                    docker!("push", &tag_url).output()?;

                    let mut mtx = st.lock().await;
                    mtx.refresh_projects()?;
                    let remote_image: RemoteImage = mtx.selected_image().try_into()?;
                    mtx.store.push_remote_image(&project_name, remote_image)?;
                    mtx.loading = None;

                    Ok(())
                });

                let st = state.clone();
                handle_spawn_error!(st, handle);
            }
            CommandBehaviour::Interactive => {
                let _cmd = ExternalCommand::init(terminal);
                docker!("image", "tag", &image_id, &tag_url)
                    .spawn()?
                    .wait()?;
                docker!("push", &tag_url).spawn()?.wait()?;
                mtx.refresh_projects()?;
                let remote_image: RemoteImage = mtx.selected_image().try_into()?;
                mtx.store.push_remote_image(&project_name, remote_image)?;
            }
        }
    }

    Ok(())
});

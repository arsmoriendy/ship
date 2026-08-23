use crate::{
    image::Image,
    tui::{config::CommandBehaviour, external_command::ExternalCommand},
};

add_app_action!(pull_image, state, terminal, {
    let mtx = state.lock().await;
    let project = mtx.selected_project();
    let registry = mtx.try_get_selected_registry()?.clone();
    let project_name = project.name.clone();
    let image = mtx.selected_image().clone();
    let behaviour = mtx.config.command_behaviours.pull_image.clone();
    drop(mtx);

    let Image::Remote(remote_image) = image else {
        return Err(anyhow!("Cannot pull local image"));
    };

    let Some(first_tag) = remote_image.tags.first() else {
        return Err(anyhow!("Image has not been tagged"));
    };

    let full_image_name = format!("{registry}/{project_name}:{first_tag}");

    match behaviour {
        CommandBehaviour::Async => {
            let mut mtx = state.lock().await;
            mtx.loading = Some("Pulling image".to_owned());
            drop(mtx);

            let st = state.clone();
            let full_image_name = full_image_name.clone();
            let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
                docker!("pull", &full_image_name).output()?;

                let mut mtx = st.lock().await;
                mtx.loading = None;

                Ok(())
            });

            let st = state.clone();
            handle_spawn_error!(st, handle);
        }
        CommandBehaviour::Interactive => {
            let _cmd = ExternalCommand::init(terminal);
            docker!("pull", &full_image_name).spawn()?.wait()?;
        }
    };

    for tag in remote_image.tags.iter().skip(1) {
        docker!(
            "tag",
            &full_image_name,
            &format!("{registry}/{project_name}:{tag}")
        )
        .output()?;
    }

    let mut mtx = state.lock().await;
    mtx.refresh_projects()?;

    Ok(())
});

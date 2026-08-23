use crate::{
    image::{RawRemoteImage, RemoteImage},
    tui::external_command::ExternalCommand,
};

add_app_action!(fetch_images, state, {
    let mtx = state.lock().await;
    let mut cmd = mtx
        .selected_cmds()
        .ok_or(anyhow!("Unconfigured registry or registry commands"))?
        .fetch_images
        .clone();

    // replace fields
    let project_name = mtx.selected_project().name.clone();
    cmd = cmd.replace("{project}", &project_name);

    drop(mtx);

    let st = state.clone();
    let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
        let mut mtx = st.lock().await;
        mtx.loading = Some("Fetching images...".to_owned());
        drop(mtx);

        let res = ExternalCommand::sh(&cmd).with_context(|| "Failed to run list images command")?;
        let raw_images: Vec<RawRemoteImage> = serde_json::from_str(res.as_str())
            .with_context(|| format!("Failed parsing list images command result: \"{res}\""))?;
        let images: Vec<RemoteImage> = raw_images
            .into_iter()
            .map(|r| r.try_into())
            .collect::<Result<_, _>>()
            .with_context(|| "Failed to parse remote images from the list images command")?;

        let mut mtx = st.lock().await;
        mtx.store.sync(|store| {
            store
                .project_remote_images
                .insert(project_name.clone(), images);
            Ok(())
        })?;
        mtx.loading = None;
        mtx.sync_project_images_from_store(project_name.as_str())?;

        Ok(())
    });

    let st = state.clone();
    handle_spawn_error!(st, handle);

    Ok(())
});

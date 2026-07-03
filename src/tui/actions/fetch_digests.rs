use crate::tui::external_command::ExternalCommand;

add_app_action!(fetch_digests, state, {
    let mtx = state.lock().await;
    let mut cmd = mtx
        .selected_cmds()
        .ok_or(anyhow!("Unconfigured registry or registry commands"))?
        .list_digests
        .clone();

    // replace fields
    let project_name = mtx.selected_project().name.clone();
    cmd = cmd.replace("{project}", &project_name);

    drop(mtx);

    let st = state.clone();
    let handle: JoinHandle<Result<()>> = tokio::spawn(async move {
        let mut mtx = st.lock().await;
        mtx.loading = Some("Fetching digests...".to_owned());
        drop(mtx);

        let res = ExternalCommand::sh(&cmd)?;
        let prefixed_digests: Vec<String> = serde_json::from_str(res.as_str())?;
        let mut digests: Vec<[u8; 32]> = vec![];
        for pd in prefixed_digests {
            digests.push(parse_prefixed_sha256(&pd)?);
        }

        let mut mtx = st.lock().await;
        mtx.store.sync(|store| {
            store.project_registry_digests.insert(project_name, digests);
            Ok(())
        })?;
        mtx.loading = None;

        Ok(())
    });

    let st = state.clone();
    handle_spawn_error!(st, handle);

    Ok(())
});

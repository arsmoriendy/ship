add_app_action!(slf; prune_images, terminal, {
    let mtx = slf.state.lock().await;
    let images = mtx.selected_project().images.clone();
    let behaviour = mtx.config.command_behaviours.delete_image.clone();
    drop(mtx);

    for image in &images {
        match behaviour {
            crate::tui::config::CommandBehaviour::Async =>
                slf._delete_image(image, None).await?,
            crate::tui::config::CommandBehaviour::Interactive =>
                slf._delete_image(image, Some(terminal)).await?,
        }
    }

    Ok(())
});

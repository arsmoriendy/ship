use crate::image::Image;

add_app_action!(slf; prune_images, terminal, {
    let mtx = slf.state.lock().await;
    let Some(digests)=mtx.store.project_registry_digests.get(mtx.selected_project().name.as_str())else { return Ok(()) };
    let images: Vec<Image> = mtx.selected_project().images.clone().into_iter().filter(|i| {
        let Some(digest)=i.digest else {return false};
        digests.contains(&digest)
    }).collect();
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

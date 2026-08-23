use crate::image::{Image, LocalImage};

add_app_action!(slf; prune_remote_images, terminal, {
    let mtx = slf.state.lock().await;
    let Some(remote_images)=mtx.store.project_remote_images.get(mtx.selected_project().name.as_str())else { return Ok(()) };
    let images: Vec<LocalImage> = mtx.selected_project().images.clone().into_iter().filter_map(|i| {
        let Image::Local(i)=i else {return None};
        let Some(digest)=i.digest else {return None};
        if remote_images.iter().any(|ri|ri.digest==digest) {return Some(i)}else {None}
    }).collect();
    let behaviour = mtx.config.command_behaviours.delete_image.clone();
    drop(mtx);

    for image in &images {
        match behaviour {
            crate::tui::config::CommandBehaviour::Async =>
                slf._delete_remote_image(image, None).await?,
            crate::tui::config::CommandBehaviour::Interactive =>
                slf._delete_remote_image(image, Some(terminal)).await?,
        }
    }

    Ok(())
});

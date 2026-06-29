add_app_action!(quit, state, {
    let mut mtx = state.lock().await;
    mtx.exit = true;
    Ok(())
});

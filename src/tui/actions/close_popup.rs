add_app_action!(close_popup, state, {
    let mut mtx = state.lock().await;
    mtx.popup = None;
    Ok(())
});

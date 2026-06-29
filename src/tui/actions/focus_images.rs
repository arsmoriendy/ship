use crate::tui::state::Focus;

add_app_action!(focus_images, state, {
    let mut mtx = state.lock().await;
    mtx.focus = Focus::Images;
    Ok(())
});

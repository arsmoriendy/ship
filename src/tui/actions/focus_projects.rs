use crate::tui::state::Focus;

add_app_action!(focus_projects, state, {
    let mut mtx = state.lock().await;
    mtx.focus = Focus::Projects;
    Ok(())
});

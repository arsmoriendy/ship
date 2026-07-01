use crate::tui::state::Focus;

add_app_action!(select_down, state, {
    let mut mtx = state.lock().await;
    match mtx.focus {
        Focus::Projects => mtx.project_table_state.select_next(),
        Focus::Images => mtx.image_table_state.select_next(),
    }
    Ok(())
});

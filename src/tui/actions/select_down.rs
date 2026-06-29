use crate::tui::state::Focus;

add_app_action!(select_down, state, {
    let mut mtx = state.lock().await;
    match mtx.focus {
        Focus::Projects => {
            mtx.selected_project = mtx
                .selected_project
                .saturating_add(1)
                .clamp(0, mtx.projects.len().saturating_sub(1));
            mtx.selected_image = 0;
        }
        Focus::Images => {
            mtx.selected_image = mtx
                .selected_image
                .saturating_add(1)
                .clamp(0, mtx.selected_project().images.len().saturating_sub(1))
        }
    }
    Ok(())
});

use crate::tui::state::Focus;

add_app_action!(close_popup, state, {
    let mut mtx = state.lock().await;
    let Focus::Popup(focus) = mtx.focus.clone() else {
        return Ok(());
    };
    mtx.focus = *focus;
    mtx.popup = None;
    Ok(())
});

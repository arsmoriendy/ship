use crate::tui::state::Focus;

mod delete_image;
mod fetch_digests;
mod focus_images;
mod focus_projects;
mod push_image;
mod quit;
mod select_down;
mod select_up;

#[derive(Clone, Debug)]
pub enum Action {
    SelectUp,
    SelectDown,
    Focus(Focus),
    PushImage,
    DeleteImage,
    FetchDigests,
    Quit,
    Noop,
}

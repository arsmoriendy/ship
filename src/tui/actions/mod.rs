use crate::prelude::*;

mod close_popup;
mod delete_image;
mod fetch_digests;
mod focus_images;
mod focus_projects;
mod push_image;
mod quit;
mod select_down;
mod select_up;

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    SelectUp,
    SelectDown,
    FocusImages,
    FocusProjects,
    PushImage,
    DeleteImage,
    FetchDigests,
    Quit,
    Noop,
    ClosePopup,
}

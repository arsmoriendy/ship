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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, strum::Display)]
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

/*
 * Action groups, per focus
 */

pub const GLOBAL_ACTIONS: [Action; 3] = [Action::SelectUp, Action::SelectDown, Action::Quit];

pub const IMAGE_ACTIONS: [Action; 3] = [
    Action::FocusProjects,
    Action::PushImage,
    Action::DeleteImage,
];

pub const PROJECT_ACTIONS: [Action; 2] = [Action::FocusImages, Action::FetchDigests];

pub const POPUP_ACTIONS: [Action; 1] = [Action::ClosePopup];

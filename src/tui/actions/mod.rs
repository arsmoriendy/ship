use crate::prelude::*;

mod close_popup;
mod delete_image;
mod focus_images;
mod focus_projects;
mod list_images;
mod prune_images;
mod pull_image;
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
    PullImage,
    DeleteImage,
    ListImages,
    PruneImages,
    Quit,
    Noop,
    ClosePopup,
}

/*
 * Action groups, per focus
 */

pub const GLOBAL_ACTIONS: [Action; 3] = [Action::SelectUp, Action::SelectDown, Action::Quit];

pub const IMAGE_ACTIONS: [Action; 4] = [
    Action::FocusProjects,
    Action::PushImage,
    Action::PullImage,
    Action::DeleteImage,
];

pub const PROJECT_ACTIONS: [Action; 3] =
    [Action::FocusImages, Action::ListImages, Action::PruneImages];

pub const POPUP_ACTIONS: [Action; 1] = [Action::ClosePopup];

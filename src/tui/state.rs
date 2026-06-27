use crate::{config::Config, project::Project, store::Store};

#[derive(PartialEq)]
pub enum Focus {
    Projects,
    Images,
}

pub struct AppState {
    pub projects: Vec<Project>,
    pub config: Config,
    pub store: Store,

    pub loading: Option<String>,
    pub spinner_frame: usize,
    pub exit: bool,

    pub selected_project: usize,
    pub selected_image: usize,

    pub focus: Focus,
}

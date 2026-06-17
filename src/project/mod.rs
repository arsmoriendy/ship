use crate::{image::Image, prelude::*};

pub struct Project {
    pub name: String,
    pub images: Vec<Image>,
}

impl Project {
    fn new(name: &str) -> Self {
        Project {
            name: name.to_owned(),
            images: vec![],
        }
    }

    pub fn get_project_name(repository: &str) -> Result<&str> {
        Ok(repository
            .split("/")
            .last()
            .ok_or(anyhow!("failed parsing project name: {}", repository))?)
    }

    pub fn list() -> Result<Vec<Project>> {
        let mut projects: Vec<Project> = vec![];
        let images = Image::list()?;
        for img in images {
            let repo = img.repository.clone();
            let project_name = Project::get_project_name(repo.as_str())?;
            if let Some(project) = projects.iter_mut().find(|p| p.name == project_name) {
                project.images.push(img)
            } else {
                let mut new_project = Project::new(project_name);
                new_project.images.push(img);
                projects.push(new_project);
            }
        }
        Ok(projects)
    }
}

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
        let mut project_names: HashMap<String, Project> = HashMap::new();
        let images = Image::list()?;
        for img in images {
            let repo = img.repository.clone();
            let project_name = Project::get_project_name(repo.as_str())?;
            if let Some(project) = project_names.get_mut(project_name) {
                project.images.push(img)
            } else {
                let mut new_project = Project::new(project_name);
                new_project.images.push(img);
                project_names.insert(project_name.to_owned(), new_project);
            }
        }
        let mut projects = project_names.into_values().collect::<Vec<Project>>();
        projects.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(projects)
    }
}

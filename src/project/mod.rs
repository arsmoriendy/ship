use crate::{image::Image, prelude::*};

struct Project {
    name: String,
    images: Vec<Image>,
}

impl Project {
    fn new(name: &str) -> Self {
        Project {
            name: name.to_owned(),
            images: vec![],
        }
    }

    fn get_project_name(repository: &str) -> Result<&str> {
        Ok(repository
            .split("/")
            .last()
            .ok_or(anyhow!("failed parsing project name: {}", repository))?)
    }

    pub fn list() -> Result<Vec<Project>> {
        let mut project_names: HashMap<String, Project> = HashMap::new();
        let images = Image::list()?;
        for img in images {
            let project_name = Project::get_project_name(img.repository.as_str())?;
            if let Some(project) = project_names.get_mut(project_name) {
                project.images.push(img)
            } else {
                project_names.insert(project_name.to_owned(), Project::new(project_name));
            }
        }
        Ok(project_names.into_values().collect::<Vec<Project>>())
    }
}

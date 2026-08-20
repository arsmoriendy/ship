use crate::{
    image::{Image, LocalImage, RemoteImage},
    prelude::*,
};

#[derive(Clone)]
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
        repository.split("/").last().ok_or(anyhow!(
            "Failed parsing project name from repository: \"{}\"",
            repository
        ))
    }

    pub fn list() -> Result<Vec<Project>> {
        let mut projects: Vec<Project> = vec![];
        let images = LocalImage::list()?;
        for img in images {
            let repo = img.repository.clone();
            let project_name = Project::get_project_name(repo.as_str())?;
            if let Some(project) = projects.iter_mut().find(|p| p.name == project_name) {
                project.images.push(Image::Local(img))
            } else {
                let mut new_project = Project::new(project_name);
                new_project.images.push(Image::Local(img));
                projects.push(new_project);
            }
        }
        Ok(projects)
    }

    pub fn find_image_with_id(&self, id: &crate::image::Id) -> Option<&Image> {
        self.images.iter().find(|img| match img {
            Image::Local(img) => &img.id == id,
            _ => false,
        })
    }

    pub fn try_find_image_with_id(&self, id: &crate::image::Id) -> Result<&Image> {
        self.find_image_with_id(id)
            .ok_or(anyhow!("Cannot find image"))
    }

    pub fn try_find_local_image_with_id(&self, id: &crate::image::Id) -> Result<&LocalImage> {
        match self.try_find_image_with_id(id)? {
            Image::Local(local_img) => Ok(local_img),
            _ => Err(anyhow!("Image is not local")),
        }
    }

    pub fn merge_remote_image(&mut self, rimg: &RemoteImage) {
        let state_images = &mut self.images;

        let state_img = state_images.iter_mut().find(|img| match img {
            Image::Local(img) => match img.digest {
                Some(digest) => digest == rimg.digest,
                None => false,
            },
            Image::Remote(img) => img.digest == rimg.digest,
        });

        match state_img {
            Some(img) => rimg.merge_with(img),
            None => state_images.push(Image::Remote(rimg.clone())),
        };
    }
}

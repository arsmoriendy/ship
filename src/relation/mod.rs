use crate::{image::Image, prelude::*, project::Project};

pub fn push<'a>(img: &'a Image, reg: &'a str) -> Result<()> {
    let id_str = encode_hex(img.id);
    let project_name = Project::get_project_name(&img.repository)?;
    let project_url = format!("{}/{}", reg, project_name);

    for tag in &img.tags {
        let tag_url = format!("{}:{}", project_url, tag);
        docker!("image", "tag", &id_str, &tag_url).output()?;
        docker!("push", &tag_url).spawn()?.wait_with_output()?;
    }

    Ok(())
}

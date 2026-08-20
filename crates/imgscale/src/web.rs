use crate::{Error, Result, model::GeneratedImage};
use std::path::{self, Path};

#[rustfmt::skip]
pub fn build_img_tag(images: &[GeneratedImage], sizes: Option<&str>, alt: Option<&str>) -> Result<String> {
    let srcset = build_srcset(images);
    let last_image = images.last().ok_or(Error::EmptyImageList)?;
    let fallback_src = last_image.web_url();

    let sizes = match sizes {
        Some(s) => s.into(),
        None => {
            let max_width = last_image.width();
            format!("(max-width: {max_width}px) 100vw, {max_width}px")
        }
    };

    let alt = alt.unwrap_or("INSERT ALT TEXT HERE");

Ok(format!(
r###"<img
  srcset="{}"
  sizes="{}"
  src="{}"
  loading="lazy" decoding="async"
  alt="{}"
>"###,
srcset, sizes, fallback_src, alt,
))
}

fn build_srcset(images: &[GeneratedImage]) -> String {
    images
        .iter()
        .map(|img| img.as_srcset_entry())
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) fn resolve_web_url(file_path: &Path, webserver_root: Option<&Path>) -> Result<String> {
    let webserver_root = match webserver_root {
        Some(root) => root,
        None => {
            let filename = file_path
                .file_name()
                .ok_or_else(|| Error::FilenameExtraction(file_path.to_path_buf()))?
                .to_str()
                .ok_or_else(|| Error::InvalidUtf8(format!("{:?}", file_path)))?;
            return Ok(filename.to_string());
        }
    };

    let absolute_file = path::absolute(file_path)?;
    let absolute_root = path::absolute(webserver_root)?;

    let relative_path =
        absolute_file
            .strip_prefix(&absolute_root)
            .map_err(|_| Error::FileOutsideRoot {
                file: absolute_file.clone(),
                root: absolute_root,
            })?;

    let mut web_url = String::new();
    for component in relative_path.components() {
        if let path::Component::Normal(segment) = component {
            if let Some(segment_str) = segment.to_str() {
                web_url.push('/');
                web_url.push_str(segment_str);
            } else {
                return Err(Error::InvalidUtf8(format!("{:?}", relative_path)));
            }
        }
    }

    if web_url.is_empty() {
        Ok("/".to_string())
    } else {
        Ok(web_url)
    }
}

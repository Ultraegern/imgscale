use crate::model::GeneratedImage;
use std::{
    io,
    path::{self, Path},
};

#[rustfmt::skip]
pub fn build_img_tag(images: &[GeneratedImage], sizes: Option<&str>, alt: Option<&str>) -> String {
    let srcset = build_srcset(images);
    let fallback_src = &images.last().unwrap().web_url();

    let sizes = match sizes {
        Some(s) => s.into(),
        None => {
            let max_width = images.last().unwrap().width();
            format!("(max-width: {max_width}px) 100vw, {max_width}px")
        }
    };

    let alt = alt.unwrap_or("INSERT ALT TEXT HERE");

format!(
r###"<img
  srcset="{}"
  sizes="{}"
  src="{}"
  loading="lazy" decoding="async"
  alt="{}"
>"###,
srcset, sizes, fallback_src, alt,
)
}

fn build_srcset(images: &[GeneratedImage]) -> String {
    images
        .iter()
        .map(|img| img.as_srcset_entry())
        .collect::<Vec<String>>()
        .join(", ")
}

pub(crate) fn resolve_web_url(
    file_path: &Path,
    webserver_root: Option<&Path>,
) -> io::Result<String> {
    let webserver_root = match webserver_root {
        Some(root) => root,
        None => {
            let filename = file_path
                .file_name()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!(
                            "Could not extract filename from path: {}",
                            file_path.display()
                        ),
                    )
                })?
                .to_str()
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Filename in path '{}' contains invalid UTF-8",
                            file_path.display()
                        ),
                    )
                })?;
            return Ok(filename.to_string());
        }
    };

    let canonical_file = file_path.canonicalize()?;
    let canonical_root = webserver_root.canonicalize()?;

    let relative_path = canonical_file.strip_prefix(&canonical_root).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "File '{}' is outside the webserver root '{}'",
                canonical_file.display(),
                canonical_root.display()
            ),
        )
    })?;

    let mut web_url = String::new();
    for component in relative_path.components() {
        if let path::Component::Normal(segment) = component {
            if let Some(segment_str) = segment.to_str() {
                web_url.push('/');
                web_url.push_str(segment_str);
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Path '{}' contains invalid UTF-8 characters",
                        relative_path.display()
                    ),
                ));
            }
        }
    }

    if web_url.is_empty() {
        Ok("/".to_string())
    } else {
        Ok(web_url)
    }
}

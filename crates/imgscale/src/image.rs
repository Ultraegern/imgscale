use crate::{Config, Error, Result, config::TargetWidth, model::GeneratedImage, web};
use image::{DynamicImage, imageops::FilterType};
use std::{fs, path::Path};

/// Loads an image from a file path, extracts its name, and scales it according to the config.
pub fn scale_image_from_file<P: AsRef<Path>>(
    path: P,
    config: &Config,
) -> Result<Vec<GeneratedImage>> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(Error::InputNotFound(path.to_path_buf()));
    }

    fs::create_dir_all(config.out_dir())?;

    let name = path
        .file_stem()
        .ok_or_else(|| Error::FilenameExtraction(path.to_path_buf()))?
        .to_str()
        .ok_or_else(|| Error::InvalidUtf8(format!("{:?}", path)))?;

    let img = image::open(path)?;

    scale_image_internal(img, name, config)
}

pub fn scale_image(img: DynamicImage, name: &str, config: &Config) -> Result<Vec<GeneratedImage>> {
    scale_image_internal(img, name, config)
}

fn scale_image_internal(
    img: DynamicImage,
    name: &str,
    config: &Config,
) -> Result<Vec<GeneratedImage>> {
    let widths = config.target_widths(img.width());

    let mut generated_images = Vec::with_capacity(widths.len());
    for width in widths {
        let (resized, filename, actual_width) = match width {
            TargetWidth::Full => {
                let filename = format!("{}-full.{}", name, config.format());

                (img.clone(), filename, img.width())
            }
            TargetWidth::Scale(w) => {
                let resized = img.resize(w, u32::MAX, FilterType::Lanczos3);
                let filename = format!("{}-{}w.{}", name, w, config.format());

                (resized, filename, w)
            }
        };

        let output_path = config.out_dir().join(&filename);

        resized.save_with_format(&output_path, config.format().into())?;

        let web_url = web::resolve_web_url(&output_path, config.root_dir())?;

        generated_images.push(GeneratedImage::new(output_path, web_url, actual_width));
    }

    Ok(generated_images)
}

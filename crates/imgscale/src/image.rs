use crate::{
    Config, Error, ExportFormat, Result,
    config::{Compression, TargetWidth},
    model::GeneratedImage,
    web,
};
use image::{
    DynamicImage, ImageDecoder, ImageReader, Limits, codecs::avif::AvifEncoder,
    imageops::FilterType, metadata::Orientation,
};
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

    let mut reader = ImageReader::open(path)?;
    reader.limits(Limits::default());

    let mut decoder = reader.into_decoder()?;
    let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);

    let mut img = DynamicImage::from_decoder(decoder)?;
    img.apply_orientation(orientation);

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
        let (filename, actual_width, needs_scaling) = match width {
            TargetWidth::Full => (
                format!("{}-full.{}", name, config.format()),
                img.width(),
                false,
            ),
            TargetWidth::Scale(w) => (format!("{}-{}w.{}", name, w, config.format()), w, true),
        };

        let output_path = config.out_dir().join(&filename);
        let web_url = web::resolve_web_url(&output_path, config.root_dir())?;

        if config.should_write(&output_path)? {
            if needs_scaling {
                let resized = img.resize(actual_width, u32::MAX, FilterType::Lanczos3);
                save_image(&resized, &output_path, config.format())?;
            } else {
                save_image(&img, &output_path, config.format())?;
            }
        }

        generated_images.push(GeneratedImage::new(output_path, web_url, actual_width));
    }

    Ok(generated_images)
}

fn save_image(img: &DynamicImage, output_path: &Path, format: ExportFormat) -> Result<()> {
    match format {
        #[cfg(feature = "zenwebp-agpl")]
        ExportFormat::Webp(Compression::Lossy) => {
            let config = zenwebp::LossyConfig::new();

            let out_bytes = match img.has_alpha() {
                true => {
                    let src = img.to_rgba8();
                    let (width, height) = src.dimensions();

                    zenwebp::EncodeRequest::lossy(
                        &config,
                        &src,
                        zenwebp::PixelLayout::Rgba8,
                        width,
                        height,
                    )
                    .encode()
                    .map_err(|e| e.decompose().0)?
                }
                false => {
                    let src = img.to_rgb8();
                    let (width, height) = src.dimensions();

                    zenwebp::EncodeRequest::lossy(
                        &config,
                        &src,
                        zenwebp::PixelLayout::Rgb8,
                        width,
                        height,
                    )
                    .encode()
                    .map_err(|e| e.decompose().0)?
                }
            };

            fs::write(output_path, out_bytes)?;
        }
        #[cfg(feature = "zenwebp-agpl")]
        ExportFormat::Webp(Compression::Lossless) => {
            let config = zenwebp::LosslessConfig::new();

            let out_bytes = match img.has_alpha() {
                true => {
                    let src = img.to_rgba8();
                    let (width, height) = src.dimensions();

                    zenwebp::EncodeRequest::lossless(
                        &config,
                        &src,
                        zenwebp::PixelLayout::Rgba8,
                        width,
                        height,
                    )
                    .encode()
                    .map_err(|e| e.decompose().0)?
                }
                false => {
                    let src = img.to_rgb8();
                    let (width, height) = src.dimensions();

                    zenwebp::EncodeRequest::lossless(
                        &config,
                        &src,
                        zenwebp::PixelLayout::Rgb8,
                        width,
                        height,
                    )
                    .encode()
                    .map_err(|e| e.decompose().0)?
                }
            };

            fs::write(output_path, out_bytes)?;
        }
        #[cfg(not(feature = "zenwebp-agpl"))]
        ExportFormat::WebpLossless => {
            img.save_with_format(output_path, image::ImageFormat::WebP)?;
        }
        ExportFormat::Avif(compression) => {
            let file = fs::File::create(output_path)?;

            let quality = match compression {
                Compression::Lossy => 80,
                Compression::Lossless => 100,
            };

            let encoder = AvifEncoder::new_with_speed_quality(file, 3, quality);

            img.write_with_encoder(encoder)?;
        }
        ExportFormat::Png => {
            img.save_with_format(output_path, image::ImageFormat::Png)?;
        }
        ExportFormat::Jpeg | ExportFormat::Jpg => {
            img.save_with_format(output_path, image::ImageFormat::Jpeg)?;
        }
    }

    Ok(())
}

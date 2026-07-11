use clap::{Parser, ValueEnum};
use image::{ImageFormat, imageops::FilterType};
use std::{num::NonZeroU32, path::PathBuf, process};

#[derive(Parser, Debug)]
#[command(name = "imgscale", version, about)]
struct Args {
    /// Path to the source image file
    input: PathBuf,

    /// Target widths for the output images (e.g. "-w 400,800,1200")
    #[arg(short, long, required = true, value_delimiter = ',')]
    widths: Vec<NonZeroU32>,

    /// Export format
    #[arg(short, long, default_value = "webp")]
    format: ExportFormat,

    /// Path to the output directory
    #[arg(short, long, default_value = ".")]
    output_dir: PathBuf,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum ExportFormat {
    Webp,
    Avif,
    Png,
    Jpeg,
    Jpg,
}

impl From<ExportFormat> for ImageFormat {
    fn from(format: ExportFormat) -> Self {
        match format {
            ExportFormat::Webp => ImageFormat::WebP,
            ExportFormat::Avif => ImageFormat::Avif,
            ExportFormat::Png => ImageFormat::Png,
            ExportFormat::Jpeg | ExportFormat::Jpg => ImageFormat::Jpeg,
        }
    }
}

impl From<ExportFormat> for &str {
    fn from(format: ExportFormat) -> Self {
        match format {
            ExportFormat::Webp => "webp",
            ExportFormat::Avif => "avif",
            ExportFormat::Png => "png",
            ExportFormat::Jpeg => "jpeg",
            ExportFormat::Jpg => "jpg",
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.input.exists() {
        eprintln!("Input file does not exist");
        process::exit(1);
    }

    let file_stem = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    println!("Reading original image: {:?}...", args.input);
    let img = image::open(&args.input)?;
    let orig_width = img.width();
    let orig_height = img.height();
    println!("Original dimensions: {}x{}", orig_width, orig_height);

    if !args.output_dir.exists() {
        std::fs::create_dir_all(&args.output_dir)?;
    }

    let target_format: ImageFormat = args.format.into();
    let extension: &str = args.format.into();

    for width in args.widths {
        let height = ((orig_height as u64 * width.get() as u64) / orig_width as u64) as u32;

        println!("Scaling to {}x{}...", width, height);

        let resized = img.resize(width.get(), height, FilterType::Lanczos3);

        let filename = format!("{}-{}w.{}", file_stem, width, extension);
        let output_path = args.output_dir.join(&filename);

        println!("Saving: {}...", filename);
        resized.save_with_format(&output_path, target_format)?;
        println!("Saved: {:?}", output_path);
    }

    println!("All sizes successfully generated!");
    Ok(())
}

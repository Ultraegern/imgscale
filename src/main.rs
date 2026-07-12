use clap::{Parser, ValueEnum};
use image::{ImageFormat, imageops::FilterType};
use std::{collections::BTreeSet, fmt, num::NonZeroU32, path::PathBuf, process};

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

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ext = match self {
            ExportFormat::Webp => "webp",
            ExportFormat::Avif => "avif",
            ExportFormat::Png => "png",
            ExportFormat::Jpeg => "jpeg",
            ExportFormat::Jpg => "jpg",
        };
        write!(f, "{}", ext)
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
    println!("Original dimensions: {}x{}", img.width(), img.height());

    if !args.output_dir.exists() {
        std::fs::create_dir_all(&args.output_dir)?;
    }

    let target_widths = {
        let mut set = BTreeSet::new();
        let mut added_full = false;
        for width in &args.widths {
            match (width.get() >= img.width(), added_full) {
                (true, false) => {
                    set.insert(width.get());
                    added_full = true;
                }
                (true, true) => {}
                (false, _) => {
                    set.insert(width.get());
                }
            }
        }

        set
    };

    let target_format: ImageFormat = args.format.into();

    for width in target_widths {
        let (resized, filename) = if width >= img.width() {
            println!(
                "Requested width ({}px) is bigger than original width ({}px). Skipping resize.",
                width,
                img.width()
            );

            let filename = format!("{}-full.{}", file_stem, args.format);

            (img.clone(), filename)
        } else {
            println!("Scaling to width {}px...", width);

            let resized = img.resize(width, u32::MAX, FilterType::Lanczos3);
            let filename = format!("{}-{}w.{}", file_stem, width, args.format);

            (resized, filename)
        };

        let output_path = args.output_dir.join(&filename);

        println!("Saving: {}...", filename);
        resized.save_with_format(&output_path, target_format)?;
        println!("Saved: {:?}", output_path);
    }

    println!("All sizes successfully generated!");
    Ok(())
}

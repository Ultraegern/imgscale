use clap::Parser;
use imgscale::{CacheMode, Config, ExportFormat, build_img_tag, scale_image_from_file};
use std::{num::NonZeroU32, path::PathBuf};

#[derive(Parser, Debug)]
#[command(name = "imgscale", version, about)]
struct Args {
    /// Path to the source image file
    input: PathBuf,

    /// Export format
    #[arg(short, long, default_value = "webp-lossy")]
    format: ExportFormat,

    /// Target widths for the output images (e.g. "-w 400,800,1200")
    #[arg(short, long, required = true, value_delimiter = ',')]
    widths: Vec<NonZeroU32>,

    /// Text to put in the <img sizes> field
    #[arg(short, long)]
    sizes: Option<String>,

    /// Text to put in the <img alt> field
    #[arg(short, long)]
    alt: Option<String>,

    /// The web server root directory. Used to generate the correct url paths (e.g. "/var/www/html" or "./dist")
    #[arg(short, long)]
    root: Option<PathBuf>,

    /// Where to save the images (e.g. "/var/www/html/assets" or "./dist/images")
    #[arg(short, long, default_value = ".")]
    output_dir: PathBuf,

    /// How should imgscale cache the output files
    #[arg(short, long, default_value = "overwrite")]
    cache_mode: CacheMode,
}

fn main() {
    let args = Args::parse();

    let config = Config::new(
        args.format,
        args.widths.into_iter().map(|w| w.get()).collect(),
        args.output_dir,
        args.root,
        args.cache_mode,
    );

    let generated_images = match scale_image_from_file(args.input, &config) {
        Ok(imgs) => imgs,
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::exit(1);
        }
    };

    let img_tag = match build_img_tag(
        &generated_images,
        args.sizes.as_deref(),
        args.alt.as_deref(),
    ) {
        Ok(tag) => tag,
        Err(e) => {
            eprintln!("ERROR: {e}");
            std::process::exit(1);
        }
    };
    println!("{}", img_tag);
}

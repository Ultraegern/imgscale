use clap::{Parser, ValueEnum};
use image::{ImageFormat, imageops::FilterType};
use std::{
    cmp,
    collections::BTreeSet,
    fmt, io,
    num::NonZeroU32,
    path::{self, Path, PathBuf},
    process,
};

#[derive(Parser, Debug)]
#[command(name = "imgscale", version, about)]
struct Args {
    /// Path to the source image file
    input: PathBuf,

    /// Export format
    #[arg(short, long, default_value = "webp")]
    format: ExportFormat,

    /// Target widths for the output images (e.g. "-w 400,800,1200")
    #[arg(short, long, required = true, value_delimiter = ',')]
    widths: Vec<NonZeroU32>,

    /// Text put in the <img sizes> field
    #[arg(short, long)]
    sizes: Option<String>,

    /// The web server root directory. Used to generate the correct paths (e.g. "/var/www/html" or "./dist")
    #[arg(short, long)]
    root: Option<PathBuf>,

    /// Where to save the images (e.g. "/var/www/html/assets" or "./dist/images")
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

#[derive(Debug)]
struct GeneratedImage {
    #[allow(unused)]
    file_path: PathBuf,
    web_url: String,
    width: u32,
}

impl GeneratedImage {
    pub fn new(file_path: PathBuf, web_url: String, width: u32) -> Self {
        Self {
            file_path,
            web_url,
            width,
        }
    }

    pub fn as_srcset_entry(&self) -> String {
        format!("{} {}w", self.web_url, self.width)
    }
}

impl PartialEq for GeneratedImage {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width
    }
}

impl Eq for GeneratedImage {}

impl PartialOrd for GeneratedImage {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.width.partial_cmp(&other.width)
    }
}

impl Ord for GeneratedImage {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.width.cmp(&other.width)
    }
}

fn build_srcset(images: &[GeneratedImage]) -> String {
    images
        .iter()
        .map(|img| img.as_srcset_entry())
        .collect::<Vec<String>>()
        .join(", ")
}

fn resolve_web_url(file_path: &Path, webserver_root: Option<&Path>) -> io::Result<String> {
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

#[rustfmt::skip]
fn build_img_tag(images: &[GeneratedImage], sizes: Option<&str>) -> String {
    let srcset = build_srcset(images);
    let fallback_src = &images.last().unwrap().web_url;

    let sizes = match sizes {
        Some(s) => s.into(),
        None => {
            let max_width = images.last().unwrap().width;
            format!("(max-width: {max_width}px) 100vw, {max_width}px")
        }
    };

format!(
r###"<img
  srcset="{}"
  sizes="{}"
  src="{}"
  loading="lazy" decoding="async"
  alt="INSERT ALT TEXT HERE"
>"###,
srcset, sizes, fallback_src
)
}

fn main() {
    let args = Args::parse();

    if !args.input.exists() {
        eprintln!("ERROR Input file does not exist");
        process::exit(1);
    }

    let file_stem = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    eprintln!("INFO Reading original image: {:?}...", args.input);
    let img = match image::open(&args.input) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("ERROR Failed to open image: {}", e);
            process::exit(1);
        }
    };

    eprintln!("INFO Original dimensions: {}x{}", img.width(), img.height());

    if !args.output_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&args.output_dir) {
            eprintln!("ERROR Failed to create output directory: {}", e);
            process::exit(1);
        }
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

    let mut generated_images = Vec::with_capacity(target_widths.len());

    for target_width in target_widths {
        let (resized, filename, actual_width) = if target_width >= img.width() {
            eprintln!(
                "WARN Requested width ({}px) is bigger than original width ({}px). Skipping resize.",
                target_width,
                img.width()
            );

            let filename = format!("{}-full.{}", file_stem, args.format);

            (img.clone(), filename, img.width())
        } else {
            eprintln!("INFO Scaling to width {}px...", target_width);

            let resized = img.resize(target_width, u32::MAX, FilterType::Lanczos3);
            let filename = format!("{}-{}w.{}", file_stem, target_width, args.format);

            (resized, filename, target_width)
        };

        let output_path = args.output_dir.join(&filename);

        eprintln!("INFO Saving: {}...", filename);
        if let Err(e) = resized.save_with_format(&output_path, target_format) {
            eprintln!("ERROR Failed to save image {}: {}", filename, e);
            process::exit(1);
        }
        eprintln!("INFO Saved: {:?}", output_path);

        let web_url = match resolve_web_url(&output_path, args.root.as_deref()) {
            Ok(url) => url,
            Err(e) => {
                eprintln!(
                    "ERROR Failed to resolve web URL for {:?}: {}",
                    output_path, e
                );
                process::exit(1);
            }
        };

        eprintln!("INFO Web url: {}", web_url);
        generated_images.push(GeneratedImage::new(output_path, web_url, actual_width));
    }

    eprintln!("INFO All sizes successfully generated!");

    generated_images.sort();
    let img_tag = build_img_tag(&generated_images, None);
    println!("{}", img_tag);
}

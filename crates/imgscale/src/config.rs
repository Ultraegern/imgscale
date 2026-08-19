use image::ImageFormat;
use std::{
    fmt, fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct Config {
    format: ExportFormat,
    widths: Vec<u32>,
    out_dir: PathBuf,
    /// Base/root directory used to resolve relative paths.
    root_dir: Option<PathBuf>,
    cache_mode: CacheMode,
}

impl Config {
    /// Returns a new [Config].
    ///
    /// `root_dir` is the base/root directory used to resolve relative paths.
    pub fn new(
        format: ExportFormat,
        widths: Vec<u32>,
        out_dir: PathBuf,
        root_dir: Option<PathBuf>,
        cache_mode: CacheMode,
    ) -> Self {
        Self {
            format,
            widths,
            out_dir,
            root_dir,
            cache_mode,
        }
    }

    /// Returns the configured image export format.
    pub fn format(&self) -> ExportFormat {
        self.format
    }

    /// Returns a slice of the requested widths.
    pub fn widths(&self) -> &[u32] {
        &self.widths
    }

    /// Computes and returns a sorted, deduplicated list of [TargetWidth]'s
    /// based on the original image's width.
    ///
    /// If a requested width is larger than or equal to the source image's width,
    /// it will be capped at [TargetWidth::Full] to prevent upscaling.
    pub(crate) fn target_widths(&self, img_width: u32) -> Vec<TargetWidth> {
        let mut widths: Vec<TargetWidth> = self
            .widths
            .iter()
            .map(|w| TargetWidth::check_widths(img_width, *w))
            .collect();
        widths.sort();
        widths.dedup();
        widths
    }

    /// Returns the path to the output directory.
    pub fn out_dir(&self) -> &Path {
        &self.out_dir
    }

    /// Returns the root directory if it was set.
    pub fn root_dir(&self) -> Option<&Path> {
        self.root_dir.as_deref()
    }

    pub fn cache_mode(&self) -> CacheMode {
        self.cache_mode
    }

    pub(crate) fn should_write(&self, path: &Path) -> io::Result<bool> {
        self.cache_mode.should_write(path)
    }
}

/// How should imgscale handle existing files
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CacheMode {
    /// Will always generate new files and overwrite any existing files
    #[default]
    Overwrite,
    /// Skip an image if all the export files already exists
    SkipExisting,
}

impl CacheMode {
    pub(crate) fn should_write(&self, path: &Path) -> io::Result<bool> {
        match self {
            Self::Overwrite => Ok(true),
            Self::SkipExisting => Ok(!fs::exists(path)?),
        }
    }
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for CacheMode {
    fn value_variants<'a>() -> &'a [Self] {
        &[CacheMode::Overwrite, CacheMode::SkipExisting]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            CacheMode::Overwrite => clap::builder::PossibleValue::new("overwrite")
                .help("Will always generate new files and overwrite any existing files"),
            CacheMode::SkipExisting => clap::builder::PossibleValue::new("skip-existing")
                .help("Skip an image if all the export files already exists"),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TargetWidth {
    /// The requested width is smaller than the original image.
    Scale(u32),
    /// The requested width is equal to or larger than the original image.
    Full,
}

impl TargetWidth {
    fn check_widths(img_width: u32, requested_width: u32) -> TargetWidth {
        if requested_width >= img_width {
            TargetWidth::Full
        } else {
            TargetWidth::Scale(requested_width)
        }
    }
}

/// Compression mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Compression {
    #[default]
    Lossy,
    Lossless,
}

/// The only difference between [Jpeg](ExportFormat::Jpeg) and [Jpg](ExportFormat::Jpg) is whether you want a `.jpeg` or a `.jpg` extension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportFormat {
    #[cfg(not(feature = "zenwebp-agpl"))]
    WebpLossless,
    #[cfg(feature = "zenwebp-agpl")]
    Webp(Compression),
    Avif(Compression),
    Png,
    Jpeg,
    Jpg,
}

impl Default for ExportFormat {
    fn default() -> Self {
        #[cfg(not(feature = "zenwebp-agpl"))]
        let out = Self::Jpg;
        #[cfg(feature = "zenwebp-agpl")]
        let out = Self::Webp(Compression::Lossy);

        out
    }
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for ExportFormat {
    fn value_variants<'a>() -> &'a [Self] {
        {
            &[
                #[cfg(feature = "zenwebp-agpl")]
                ExportFormat::Webp(Compression::Lossy),
                #[cfg(feature = "zenwebp-agpl")]
                ExportFormat::Webp(Compression::Lossless),
                #[cfg(not(feature = "zenwebp-agpl"))]
                ExportFormat::WebpLossless,
                ExportFormat::Avif(Compression::Lossy),
                ExportFormat::Avif(Compression::Lossless),
                ExportFormat::Png,
                ExportFormat::Jpeg,
                ExportFormat::Jpg,
            ]
        }
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        use clap::builder::PossibleValue;

        match self {
            #[cfg(feature = "zenwebp-agpl")]
            ExportFormat::Webp(Compression::Lossy) => Some(
                PossibleValue::new("webp-lossy")
                    .alias("webp")
                    .help("WebP with lossy compression"),
            ),
            #[cfg(feature = "zenwebp-agpl")]
            ExportFormat::Webp(Compression::Lossless) => {
                Some(PossibleValue::new("webp-lossless").help("WebP with lossless compression"))
            }
            #[cfg(not(feature = "zenwebp-agpl"))]
            ExportFormat::WebpLossless => Some(
                PossibleValue::new("webp-lossless")
                    .alias("webp")
                    .help("WebP with lossless compression"),
            ),
            ExportFormat::Avif(Compression::Lossy) => Some(
                PossibleValue::new("avif-lossy")
                    .alias("avif")
                    .help("AVIF with lossy compression"),
            ),
            ExportFormat::Avif(Compression::Lossless) => {
                Some(PossibleValue::new("avif-lossless").help("AVIF with lossless compression"))
            }
            ExportFormat::Png => {
                Some(PossibleValue::new("png").help("Png with lossless compression"))
            }
            ExportFormat::Jpeg => {
                Some(PossibleValue::new("jpeg").help("JPEG with lossy compression"))
            }
            ExportFormat::Jpg => Some(
                PossibleValue::new("jpg").help("JPEG with lossy compression and a .jpg extension"),
            ),
        }
    }
}

impl From<ExportFormat> for ImageFormat {
    fn from(format: ExportFormat) -> Self {
        match format {
            #[cfg(not(feature = "zenwebp-agpl"))]
            ExportFormat::WebpLossless => ImageFormat::WebP,
            #[cfg(feature = "zenwebp-agpl")]
            ExportFormat::Webp(_) => ImageFormat::WebP,
            ExportFormat::Avif(_) => ImageFormat::Avif,
            ExportFormat::Png => ImageFormat::Png,
            ExportFormat::Jpeg | ExportFormat::Jpg => ImageFormat::Jpeg,
        }
    }
}

impl fmt::Display for ExportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ext = match self {
            #[cfg(not(feature = "zenwebp-agpl"))]
            ExportFormat::WebpLossless => "webp",
            #[cfg(feature = "zenwebp-agpl")]
            ExportFormat::Webp(_) => "webp",
            ExportFormat::Avif(_) => "avif",
            ExportFormat::Png => "png",
            ExportFormat::Jpeg => "jpeg",
            ExportFormat::Jpg => "jpg",
        };
        write!(f, "{}", ext)
    }
}

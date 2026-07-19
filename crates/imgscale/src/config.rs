use image::ImageFormat;
use std::{
    fmt,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug)]
pub struct Config {
    format: ExportFormat,
    widths: Vec<u32>,
    out_dir: PathBuf,
    /// Base/root directory used to resolve relative paths.
    root_dir: Option<PathBuf>,
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
    ) -> Self {
        Self {
            format,
            widths,
            out_dir,
            root_dir,
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

/// The only difference between [Jpeg](ExportFormat::Jpeg) and [Jpg](ExportFormat::Jpg) is whether you want a `.jpeg` or a `.jpg` extension.
#[derive(Clone, Copy, Debug, Default)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum ExportFormat {
    #[default]
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

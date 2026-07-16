use std::{io, path::PathBuf};

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Input file does not exist: {0}")]
    InputNotFound(PathBuf),

    #[error("Failed to extract filename or stem from path: {0}")]
    FilenameExtraction(PathBuf),

    #[error("'{0}' contains invalid UTF-8")]
    InvalidUtf8(String),

    #[error("File '{file}' is outside the root '{root}'")]
    FileOutsideRoot { file: PathBuf, root: PathBuf },

    #[error("Cannot generate HTML: No images were provided")]
    EmptyImageList,

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

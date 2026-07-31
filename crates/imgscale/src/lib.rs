//! A library for generating various sizes of an image and the appropriate html <img> tag
//!
//! ## Example
//!
//! ```toml
//! [dependencies]
//! imgscale = "0.1.0"
//! ```
//!
//! ```rust no_run
//! use std::path::PathBuf;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = imgscale::Config::new(
//!         imgscale::ExportFormat::Jpeg,
//!         vec![400, 800, 1200, 2400, 3600],
//!         PathBuf::from("dist/assets/"),
//!         Some(PathBuf::from("dist/")),
//!     );
//!
//!     let generated_images =
//!         imgscale::scale_image_from_file("src/assets/some-image.jpeg", &config)?;
//!
//!     let img_tag = imgscale::build_img_tag(&generated_images, None, Some("Just some image"))?;
//!
//!     println!("{}", img_tag);
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod error;
pub mod image;
pub mod model;
pub mod web;

pub use config::{Config, ExportFormat};
pub use error::{Error, Result};
pub use image::{scale_image, scale_image_from_file};
pub use model::GeneratedImage;
pub use web::build_img_tag;

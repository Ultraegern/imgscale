# Imgscale

A cli / library for generating various sizes of an image and the appropriate html `<img>` tag.

## Features

- Supports a wide range of *input* formats:

  - Jpeg / Jpg
  - Png
  - Webp
  - Avif

- Supports a wide range of *output* formats:

  - Jpeg / Jpg
  - Png
  - Webp Lossless
  - Webp Lossy (only with `zenwebp-agpl` feature)
  - Avif

- Generates html `<img>` tags:

  - Can resolve the image web-path from a given root dir.

- Supports output caching.

- Handles one image on a single thread.

  - You must bring your own multi-image/multithreading

## License

### imgscale

**`imgscale`** is licensed at your choice under either **MIT** or **Apache-2.0**.

> [!IMPORTANT]  
> The `zenwebp-agpl` feature pulls in `zenwebp` which is licensed **AGPL-3.0-only**.  
> Enabling the optional `zenwebp-agpl` feature subjects any compiled binary or service using it to the terms of the AGPL-3.0.

### imgscale-cli

**`imgscale-cli`** is licensed **AGPL-3.0-only**, because it pulls in `zenwebp`.

## Cli

### Installation

Right now it is only available from source. I might publish binaries to GitHub Releases and add support for `cargo binstall` in the future.

Install from source (will install the `imgscale` binary):

```shell
cargo install imgscale-cli
```

### Usage

```shell
imgscale --widths 400,800,1200 input-img.jpg --output-dir some/dir
```

## Library

See examples and documentation at [docs.rs](https://docs.rs/imgscale).

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
  - Webp (Note: Only supports Lossless)
  - Avif

- Generates html `<img>` tags:

  - Can resolve the image web-path from a given root dir.

- Handles one image on a single thread.

  - You must bring your own multi-image/multithreading

## Cli

### Installation

Right now it is only available from source. I might publice binaries to GitHub Releases and add support for `cargo binstall` in the future.

Install from source (will install the `imgscale` binary):

```shell
cargo install imgscale-cli
```

### Usage

```shell
imgscale --widths 400,800,1200 input-img.jpg --output-dir some/dir
```

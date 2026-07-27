# Imgscale

A cli / library for generating various sizes of an image and the appropriate html `<img>` tag.

## Features

- Supports a wide range of input formats: (almost every format the `image` crate supports)

  - Jepg / Jpg
  - Png
  - Webp
  - Avif

- Supports a wide range of output formats:

  - Jepg / Jpg
  - Png
  - Webp (Note: Only supports Lossless)
  - Avif

- Generates html `<img>` tags

- TODO

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

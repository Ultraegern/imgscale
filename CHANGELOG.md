# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/2.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Adds caching support. Enable/disable using `imgscale::CacheMode`. `CacheMode::Overwrite` is the same behavior as before (Always render and save all images), and `CacheMode::SkipExisting` dosn't render an image if it is already rendered.

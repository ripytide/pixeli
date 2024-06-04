# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## Unreleased - ReleaseDate

## 0.2.1 - 2024-06-04

### Added

- Added `as_mut_slice()` method to the `AsSlice` trait.

## 0.2.0 - 2024-06-04

### Added

- Added `AsSlice`, `ContiguousPixel`, `Enlargeable`, `FromPixelCommon`,
  `FromComponentCommon`, `PixelComponent`, `WithAlpha`, `WithoutAlpha`
  traits.

### Changed

- Much improved `Pixel` trait and associated types.

### Removed

- The `Gbr` pixel type

## 0.1.0 - 2024-06-01

### Added

- Added Pixel structs and `Pixel` trait

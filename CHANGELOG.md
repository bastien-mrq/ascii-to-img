# Changelog

## [2.0.0] - 2026-01-07

### Added
- Colored ASCII output support with the `--color` flag
- New Color struct for RGB color representation
- Terminal color support using owo-colors library
- Colored ASCII conversion functionality

### Changed
- Refactored image processing to support both grayscale and color modes
- Updated main.rs to handle both colored and regular ASCII output
- Improved modularity of ASCII conversion functions

### Dependencies
- Added owo-colors 4.2.3 for terminal color support

## [0.1.0] - Initial Release

### Added
- Convert images to ASCII art
- Multiple ASCII art styles (simple, detailed, block, intensity, minimalist, circle)
- Configurable output width
- Console or file output
- Aspect ratio preservation with character scaling
- Command-line interface with clap
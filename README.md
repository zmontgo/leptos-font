# Leptos Font
A simple font handler and optimizer designed to work with Leptos. Designed around sensible defaults with the ability to customize as needed.

## Features
- Load fonts from TTF, OTF, WOFF, WOFF2, and AAT
- Optimize fonts for use in Leptos through manual and comptime automatic subsetting.
- Providing CSS classes or variables for easy integration with web projects.
- Automatic conversion to WOFF2.
- Automatic preload directive injection.

## Usage
```rust
use leptos_font::{FontManager};

let mut manager = FontManager::new();
let font = manager.load_font!("path/to/font.ttf", 16.0).unwrap();
```

## License
Leptos Font is licensed under the MIT license.

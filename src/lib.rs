use base64::{engine::general_purpose, Engine};
use file_format;
use leptos::{view, IntoView};
use short_uuid;
use thiserror;

#[derive(Debug, thiserror::Error)]
enum FontError {
  #[error("File not found")]
  FileNotFound,
  #[error("File was not a valid font file")]
  InvalidFont,
}

struct Font {
  base64: String,
  format: file_format::FileFormat,
}

struct FontManager {
  fonts: Vec<Font>,
}

impl FontManager {
  /// Generate a new FontManager to add and manage fonts
  ///
  pub fn new() -> Self {
    FontManager { fonts: Vec::new() }
  }

  /// Add a font to the manager
  /// # Arguments
  /// * `path` - Path to the font file
  /// # Returns
  /// * `Result<(), FontError>` - Result indicating success or failure
  pub fn add_font(&mut self, path: &str) -> Result<(), FontError> {
    let contents = std::fs::read(path).unwrap();

    // Get file format
    let format = file_format::FileFormat::from_bytes(&contents);

    if format.kind() != file_format::Kind::Font {
      return Err(FontError::InvalidFont);
    }

    let base64 = general_purpose::STANDARD.encode(&*contents);

    self.fonts.push(Font { base64, format });

    Ok(())
  }

  /// Generate CSS style for all fonts
  fn generate_style(self) -> (impl IntoView, String) {
    let mut style = String::new();
    let font_name = short_uuid::ShortUuid::generate();
    for font in &self.fonts {
      style.push_str(&format!(
        r#"
                @font-face {{
                    font-family: '{}';
                    src: url(data:font/{};base64,{}) format('{}');
                }}
                "#,
        font_name,
        font.format.short_name().unwrap().to_lowercase(),
        font.base64,
        font.format.short_name().unwrap()
      ));
    }

    (view! { <style>{ style }</style> }, font_name.to_string())
  }
}

// #[component]
// fn Stylesheet() -> Html {
//   let font_manager = FontManager::new();
//   font_manager.add_font("path/to/font.ttf").unwrap();
//   let style = font_manager.generate_style();
//   html! {
//     <style>
//       { style }
//     </style>
//   }
// }

#[cfg(test)]
mod tests {
  use leptos::component;

  use super::*;

  #[test]
  fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
  }
}

use anyhow::Result;
use std::{collections::HashMap, path::Path};
use rusttype::Font;

/// Resources holder (in this case the holder only handle Font but it can be extended to hold textures, sounds, ..)
pub struct Assets<'a> {
    // TODO : not sure the Font struct is interesting, we could only store rusttype::Font
    fonts: HashMap<String, Font<'a>>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    /// Returns a reference to the named font
    ///
    /// # Arguments
    ///
    /// * `name` - Font name
    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    /// Load font into Assets holder
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the font file
    pub fn load_font(&mut self, path: &Path) -> Result<()> {
        let bytes = std::fs::read(path)?;
        let font = rusttype::Font::try_from_vec(bytes).unwrap();

        self.fonts.insert(
            path.file_name().unwrap().to_str().unwrap().to_string(),
            font,
        );

        Ok(())
    }
}

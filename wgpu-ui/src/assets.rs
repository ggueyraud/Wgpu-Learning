use anyhow::Result;
use std::{collections::HashMap, path::Path};

use crate::graphics::font::Font;

pub struct Assets<'a> {
    fonts: HashMap<String, Font<'a>>,
}

impl<'a> Assets<'a> {
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
        }
    }

    pub fn get_font(&self, name: &str) -> Option<&Font> {
        self.fonts.get(name)
    }

    pub fn load_font(&mut self, path: &Path) -> Result<()> {
        let font = Font::new(path)?;

        self.fonts.insert(
            path.file_name().unwrap().to_str().unwrap().to_string(),
            font,
        );

        Ok(())
    }
}

use std::{fs, path::Path};

use anyhow::Result;

pub struct Font<'a> {
    pub internal: rusttype::Font<'a>,
}

impl<'a> Font<'a> {
    pub fn new(path: &Path) -> Result<Self> {
        let bytes = fs::read(path)?;
        let font = rusttype::Font::try_from_vec(bytes).unwrap();

        Ok(Self { internal: font })
    }
}

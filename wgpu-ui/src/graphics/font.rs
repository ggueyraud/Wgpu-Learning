use std::fs;

use anyhow::Result;

pub struct Font<'a> {
    pub internal: rusttype::Font<'a>,
}

impl<'a> Font<'a> {
    pub fn new(filename: &str) -> Result<Self> {
        let bytes = fs::read(filename)?;
        let font = rusttype::Font::try_from_vec(bytes).unwrap();

        Ok(Self { internal: font })
    }
}

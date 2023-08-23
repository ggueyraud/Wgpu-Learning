// use rusttype::gpu_cache::Cache;
// use rusttype::{point, vector, PositionedGlyph, Rect, Scale};
use rusttype::Scale;
// use winit::dpi::Position;

use crate::assets::FontRes;
use crate::graphics::Drawable;
use crate::Vertex;

// pub fn render_glyphs<'a>(
//     pass: &mut wgpu::RenderPass,
//     glyphs: Vec<PositionedGlyph>,
//     cache: &Cache<'a>,
//     screen_width: u32,
//     screen_height: u32,
// ) {
//     let vertices: Vec<Vertex> = glyphs
//         .iter()
//         // .filter_map(f)
//         .filter_map(|g| cache.rect_for(0, g).ok().flatten())
//         .flat_map(|(uv_rect, screen_rect)| {
//             let origin = point(0.0, 0.0);
//             let gl_rect = Rect {
//                 min: origin
//                     + (vector(
//                         screen_rect.min.x as f32 / screen_width - 0.5,
//                         1.0 - screen_rect.min.y as f32 / screen_height - 0.5,
//                     )) * 2.0,
//                 max: origin
//                     + (vector(
//                         screen_rect.max.x as f32 / screen_width - 0.5,
//                         1.0 - screen_rect.max.y as f32 / screen_height - 0.5,
//                     )) * 2.0,
//             };

//             Vertex {
//                 // position: [uv_rect.po]
//                 tex_coords: [uv_rect.min.x],
//             }
//         })
//         .collect();

//     pass.set_vertex_buffer(0, buffer_slice)
// }

pub struct Text {
    font: FontRes,
    text: String,
    character_size: u8,
    vertices: Vec<Vertex>,
    vertex_buffer: Option<wgpu::Buffer>,
}

impl Text {
    pub fn new(text: &str, font: FontRes) -> Self {
        Self {
            text: text.to_owned(),
            font,
            character_size: 30,
            vertices: Vec::new(),
            vertex_buffer: None,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
    }

    pub fn set_character_size(&mut self, character_size: u8) {
        self.character_size = character_size;
    }

    pub fn set_font(&mut self, font: FontRes) {
        self.font = font;
    }

    pub fn update(&mut self) {
        let scale = Scale::uniform(40.0);
        // let v_metrics = *(self.font).0.
        // let glyphs = Vec<Position<'_>> = (*self.font).0.
    }

    fn create_vertices(&self) {}
}

impl Drawable for Text {
    fn draw<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
        match &self.vertex_buffer {
            Some(buffer) => {
                render_pass.set_bind_group(0, &(*self.font).1, &[]);
                // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                // render_pass.draw(0.., 0..1);
            }
            _ => (),
        }
    }
}

use super::{Color, Drawable, Transformable, Vertex, WHITE};
use glam::Vec2;

pub trait Shape: Transformable + Drawable {
    fn set_fill_color(&mut self, color: Color);
    fn get_point(&self, index: usize) -> Vec2;
    fn get_point_count(&self) -> usize;
}

pub struct RectangleShape {
    // vertex_buffer: wgpu::Buffer,
    color: Color,
    vertices: Vec<Vertex>,
    position: Vec2,
    size: Vec2,
}

impl RectangleShape {
    pub fn new(size: Vec2) -> Self {
        Self {
            position: Default::default(),
            size,
            color: WHITE,
            vertices: vec![],
        }
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    pub fn size(&self) -> &Vec2 {
        &self.size
    }

    fn update(&mut self) {
        for i in 0..self.get_point_count() {
            let point: [f32; 2] = self.get_point(i).into();

            if let Some(vertex) = self.vertices.get_mut(i + 1) {
                vertex.position = point;
            }
        }

        self.update_fill_color();

        // TODO : update buffer
    }

    fn update_fill_color(&mut self) {
        for vertex in &mut self.vertices {
            vertex.color = self.color.into();
        }
    }
}

impl Shape for RectangleShape {
    fn get_point(&self, index: usize) -> Vec2 {
        match index {
            1 => (self.size.x, 0.).into(),
            2 => self.size,
            3 => (0., self.size.y).into(),
            _ => (0., 0.).into(),
        }
    }

    fn get_point_count(&self) -> usize {
        4
    }

    fn set_fill_color(&mut self, color: Color) {
        self.color = color;

        self.update_fill_color();
    }
}

impl Transformable for RectangleShape {
    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn position(&self) -> Vec2 {
        self.position
    }
}

impl Drawable for RectangleShape {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) {
        // Set vertex buffer
        // Set index buffer
    }
}

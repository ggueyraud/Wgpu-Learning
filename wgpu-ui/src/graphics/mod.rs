use derive_more::From;
use glam::Vec2;

pub mod font;
pub mod shape;
pub mod text;

#[derive(From, Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [self.0 as f32, self.1 as f32, self.2 as f32]
    }
}

#[allow(dead_code)]
pub const BLACK: Color = Color(0, 0, 0);
pub const WHITE: Color = Color(255, 255, 255);
pub const RED: Color = Color(255, 0, 0);
pub const GREEN: Color = Color(0, 255, 0);
pub const BLUE: Color = Color(0, 0, 255);

pub trait Drawable {
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait Transformable {
    fn set_position(&mut self, position: Vec2);
    fn position(&self) -> Vec2;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

use glam::Vec2;

pub mod color;
pub mod shape;
pub mod text;

pub trait Drawable {
    /// Draw the object to the screen
    ///
    /// # Arguments
    ///
    /// * `wgpu::RenderPass` - The render pass which process the object
    fn draw<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub trait Transformable {
    /// Set the position of the object
    ///
    /// # Arguments
    ///
    /// * `position` - New object position
    fn set_position(&mut self, position: Vec2);

    /// Get the position of the object
    fn position(&self) -> &Vec2;
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

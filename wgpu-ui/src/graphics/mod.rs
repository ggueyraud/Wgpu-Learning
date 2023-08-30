use derive_more::From;
use glam::{Mat4, Quat, Vec2};
pub mod font;
pub mod text;

#[derive(From)]
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

type TransformRaw = [[f32; 4]; 4];

pub struct Transform {
    position: Vec2,
    // rotation: Quat
    rotation: f32,
}

impl Transform {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<TransformRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    // While our vertex shader only uses locations 0, and 1 now, in later tutorials we'll
                    // be using 2, 3, and 4, for Vertex. We'll start at slot 5 not conflict with them later
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    pub fn positition(&self) -> Vec2 {
        self.position
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn to_raw(&self) -> TransformRaw {
        let matrix = Mat4::from_translation((self.position, 0.).into())
            * Mat4::from_rotation_z(self.rotation);
        let mut arr: [[f32; 4]; 4] = Default::default();

        for i in 0..4 {
            arr[i] = [
                matrix.x_axis[i],
                matrix.y_axis[i],
                matrix.z_axis[i],
                matrix.w_axis[i],
            ];
        }

        arr

        // [
        //     arr
        //     [arr.x_axis.0],
        //     [0., 0., 0., 0.],
        //     [0., 0., 0., 0.],
        //     [0., 0., 0., 0.],
        // ]
    }
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

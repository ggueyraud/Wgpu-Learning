use std::sync::Arc;

use wgpu::util::DeviceExt;

use crate::{Vertex, graphics::Drawable, assets::T};

const TILE_SIZE: f32 = 0.1;
const TILE_SIZE2: f32 = 32.;

pub struct Map {
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,
    texture: T
}

impl Map {
    pub fn new(size: (u8, u8), tileset: T, tiles: &[u8], device: &wgpu::Device) -> Self {
        let mut vertices = Vec::new();
        let tileset_width = tileset.0.texture.size().width;
        println!("tileset width {}", tileset_width as u8);

        for i in 0..size.0 {
            for j in 0..size.1 {
                let index = (i + j * size.0) as usize;
                println!("index {}", index);
                let tile_number = *tiles.get(index).unwrap();
                println!("tile number {}", tile_number);
                let tu = (tile_number as u32 % tileset_width / TILE_SIZE2 as u32) as f32;
                let tv = (tile_number as u32 / tileset_width / TILE_SIZE2 as u32) as f32;

                vertices.push(Vertex {
                    position: [i as f32 * TILE_SIZE, j as f32 * TILE_SIZE],
                    tex_coords: [tu * TILE_SIZE2, tv * TILE_SIZE2]
                });
                vertices.push(Vertex {
                    position: [(i + 1) as f32 * TILE_SIZE, j as f32 * TILE_SIZE],
                    tex_coords: [(tu + 1.) * TILE_SIZE2, tv * TILE_SIZE2]
                });
                vertices.push(Vertex {
                    position: [i as f32 * TILE_SIZE, (j + 1) as f32 * TILE_SIZE],
                    tex_coords: [tu * TILE_SIZE2, (tv + 1.) * TILE_SIZE2]
                });

                vertices.push(Vertex {
                    position: [(i + 1) as f32 * TILE_SIZE, j as f32 * TILE_SIZE],
                    tex_coords: [(tu + 1.) * TILE_SIZE2, tv * TILE_SIZE2]
                });
                vertices.push(Vertex {
                    position: [(i + 1) as f32 * TILE_SIZE, (j + 1) as f32 * TILE_SIZE],
                    tex_coords: [(tu + 1.) * TILE_SIZE2, (tv + 1.) * TILE_SIZE2]
                });
                vertices.push(Vertex {
                    position: [i as f32 * TILE_SIZE, (j + 1) as f32 * TILE_SIZE],
                    tex_coords: [tu * TILE_SIZE2, (tv + 1.) * TILE_SIZE2]
                });
            }
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
        });

        Self {
            texture: tileset,
            vertices,
            vertex_buffer,
        }
    }
}

impl Drawable for Map {
    fn draw<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
        render_pass.set_bind_group(0, &(*self.texture).1, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertices.len() as _, 0..1);
    }
}
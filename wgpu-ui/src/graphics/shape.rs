use crate::{
    math::{pixels_to_clip, Rect},
    Ctx, PIPELINES,
};

use super::{
    color::{Color, WHITE},
    Drawable, Transformable, Vertex,
};
use glam::Vec2;
use wgpu::util::DeviceExt;

pub trait Shape: Transformable + Drawable {
    /// Fill all vertices with specified color
    ///
    /// # Arguments
    ///
    /// * `color` - Shape color
    fn set_fill_color(&mut self, color: Color);

    /// Returns the position of a point of the shape
    ///
    /// # Arguments
    ///
    /// * `index` - Index of the point whose position we want
    fn get_point(&self, index: usize) -> Vec2;

    /// Returns the number of points of the shape
    fn get_point_count(&self) -> usize;
}

pub struct RectangleShape {
    context: Ctx,
    vertex_buffer: wgpu::Buffer,
    color: Color,
    vertices: Vec<Vertex>,
    position: Vec2,
    size: Vec2,
}

impl RectangleShape {
    pub fn new(context: Ctx, size: Vec2) -> Self {
        let ctx = context.lock().unwrap();
        let mut vertices = Vec::new();

        for _ in 0..4 {
            vertices.push(Vertex {
                position: [0., 0.],
                color: WHITE.into(),
                tex_coords: [-1., -1.],
            });
        }

        let vertex_buffer = ctx
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });
        drop(ctx);

        let mut s = Self {
            context,
            position: Default::default(),
            size,
            color: WHITE,
            vertices,
            vertex_buffer,
        };
        s.update();

        s
    }

    pub fn bounds(&self) -> Rect {
        Rect {
            x: self.position.x,
            y: self.position.y,
            width: self.size.x,
            height: self.size.y,
        }
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
        self.update();
    }

    pub fn size(&self) -> &Vec2 {
        &self.size
    }

    fn update(&mut self) {
        let ctx = self.context.lock().unwrap();
        let screen_size = (ctx.config.width as f32, ctx.config.height as f32);
        drop(ctx);

        for i in 0..self.get_point_count() {
            let point = self.get_point(i);

            if let Some(vertex) = self.vertices.get_mut(i) {
                vertex.position = pixels_to_clip(
                    self.position.x + point.x,
                    self.position.y + point.y,
                    screen_size.0,
                    screen_size.1,
                );
            }
        }

        self.update_fill_color();

        let ctx = self.context.lock().unwrap();
        ctx.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
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
            1 => (0., self.size.y).into(),
            2 => self.size,
            3 => (self.size.x, 0.).into(),
            _ => (0., 0.).into(),
        }
    }

    fn get_point_count(&self) -> usize {
        4
    }

    fn set_fill_color(&mut self, color: Color) {
        self.color = color;

        // self.update_fill_color();
        self.update();
    }
}

impl Transformable for RectangleShape {
    fn set_position(&mut self, position: Vec2) {
        self.position = position;

        self.update();
    }

    fn position(&self) -> &Vec2 {
        &self.position
    }
}

impl Drawable for RectangleShape {
    fn draw<'b>(&'b mut self, render_pass: &mut wgpu::RenderPass<'b>) {
        render_pass.set_pipeline(&PIPELINES.get().unwrap().get("std").unwrap().0);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}

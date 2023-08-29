use std::sync::{Arc, Mutex};

use crate::graphics::Drawable;
use crate::{math::pixels_to_clip, Context, Vertex};
use glam::Vec2;
use wgpu::{util::DeviceExt, RenderPass};
use winit::event::{ElementState, WindowEvent};

pub struct Ui {
    widgets: Vec<Box<dyn Widget>>,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget)
    }

    pub fn process_events(&mut self, event: &WindowEvent) {
        self.widgets
            .iter_mut()
            .for_each(|w| w.process_events(event));
    }

    pub fn update(&mut self, dt: f32) {
        self.widgets.iter_mut().for_each(|w| w.update(dt));
    }

    pub fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        self.widgets.iter().for_each(|w| w.draw(render_pass));
    }
}

pub trait Widget: Drawable {
    fn process_events(&mut self, event: &WindowEvent);

    fn update(&mut self, dt: f32) {
        return;
    }

    fn position(&self) -> &Vec2;
}

enum ButtonState {
    None,
    Hover,
    Click,
}

pub struct Button {
    context: Arc<Mutex<Context>>,
    text: String,
    position: Vec2,
    size: Vec2,
    vertex_buffer: wgpu::Buffer,
    color: [f32; 3],
    state: ButtonState,
    vertices: Vec<Vertex>,
    mp: Vec2,
}

impl Button {
    pub fn new(text: &str, context: Arc<Mutex<Context>>) -> Self {
        let mut vertices = Vec::new();
        let color = [1., 0., 0.];
        let screen_size = {
            let c = context.lock().unwrap();
            (c.config.width as f32, c.config.height as f32)
        };
        let position = (50., 50.);

        vertices.push(Vertex {
            position: pixels_to_clip(position.0, position.1, screen_size.0, screen_size.1),
            color,
            tex_coords: [-1.0, -1.0],
        });
        vertices.push(Vertex {
            position: pixels_to_clip(position.0, position.1 + 50., screen_size.0, screen_size.1),
            color,
            tex_coords: [-1.0, -1.0],
        });
        vertices.push(Vertex {
            position: pixels_to_clip(
                position.0 + 100.,
                position.1 + 50.,
                screen_size.0,
                screen_size.1,
            ),
            color,
            tex_coords: [-1.0, -1.0],
        });
        vertices.push(Vertex {
            position: pixels_to_clip(position.0 + 100., position.1, screen_size.0, screen_size.1),
            color,
            tex_coords: [-1.0, -1.0],
        });

        let vertex_buffer = {
            let c = context.lock().unwrap();

            c.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex buffer"),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                })
        };

        Self {
            text: text.to_string(),
            vertices,
            context,
            position: position.into(),
            size: (100., 50.).into(),
            vertex_buffer,
            color,
            state: ButtonState::None,
            mp: (0., 0.).into(),
        }
    }

    fn click() {
        println!("Click event");
    }

    fn set_fill_color(&mut self, color: [f32; 3]) {
        let c = self.context.lock().unwrap();

        self.vertices.iter_mut().for_each(|v| v.color = color);
        c.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }
}

impl Widget for Button {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn update(&mut self, dt: f32) {
        // match self.state {
        //     ButtonState::Click => {
        //         let color = [0., 1., 0.];

        //         self.vertices.iter_mut().for_each(|v| v.color = color);

        //         queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        //     },
        //     ButtonState::Hover => {
        //         let color = [0., 0., 1.];

        //         self.vertices.iter_mut().for_each(|v| v.color = color);

        //         queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        //     },
        //     _ => {}
        // }
    }

    fn process_events(&mut self, event: &WindowEvent) {
        let mut s = ButtonState::None;
        let inside = |x: f32, y: f32| -> bool {
            x >= self.position.x
                && x <= self.position.x + self.size.x
                && y >= self.position.y
                && y <= self.position.y + self.size.y
        };

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = (position.x as f32, position.y as f32);
                self.mp = (x, y).into();

                if inside(x, y) {
                    self.set_fill_color([0., 1., 0.]);
                } else {
                    self.set_fill_color([1., 0., 0.]);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if inside(self.mp.x, self.mp.y) {
                    match *state {
                        ElementState::Pressed => {
                            // s = ButtonState::Click;
                            Self::click();
                            self.set_fill_color([0., 0., 1.]);
                        }
                        ElementState::Released => {
                            self.set_fill_color([0., 1., 0.]);
                        }
                    }
                }
            }
            _ => {}
        }

        self.state = s;
    }
}

impl Drawable for Button {
    fn draw<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}

use std::sync::{Arc, Mutex};

use crate::graphics::text::Text;
use crate::graphics::{Drawable, Transformable, BLUE, GREEN, RED};
use crate::ASSETS;
use crate::{math::pixels_to_clip, Context, Vertex};
use glam::{Vec2, Vec4};
use wgpu::{util::DeviceExt, RenderPass};
use winit::event::{ElementState, MouseButton, WindowEvent};

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

    pub fn draw<'a>(&'a mut self, render_pass: &mut RenderPass<'a>) {
        self.widgets.iter_mut().for_each(|w| w.draw(render_pass));
    }
}

pub trait Widget: Drawable + Transformable {
    fn process_events(&mut self, event: &WindowEvent);

    fn update(&mut self, _dt: f32) {}

    fn position(&self) -> &Vec2;
}

enum ButtonState {
    None,
    Hover,
    Click,
}

// TODO : replace all about vertex by RectangleShape
pub struct Button<'a> {
    context: Arc<Mutex<Context>>,
    label: Text<'a>,
    position: Vec2,
    size: Vec2,
    vertex_buffer: wgpu::Buffer,
    state: ButtonState,
    vertices: Vec<Vertex>,
    mp: Vec2,
    paddings: Vec4,
}

impl<'a> Transformable for Button<'a> {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.label.set_position(position);

        let c = self.context.lock().unwrap();

        let screen_size = { (c.config.width as f32, c.config.height as f32) };

        self.vertices
            .iter_mut()
            .enumerate()
            .for_each(|(index, vertex)| match index {
                0 => {
                    vertex.position =
                        pixels_to_clip(position.x, position.y, screen_size.0, screen_size.1);
                }
                1 => {
                    vertex.position = pixels_to_clip(
                        position.x,
                        position.y + self.size.y,
                        screen_size.0,
                        screen_size.1,
                    );
                }
                2 => {
                    vertex.position = pixels_to_clip(
                        position.x + self.size.x,
                        position.y + self.size.y,
                        screen_size.0,
                        screen_size.1,
                    );
                }
                3 => {
                    vertex.position = pixels_to_clip(
                        position.x + self.size.x,
                        position.y,
                        screen_size.0,
                        screen_size.1,
                    );
                }
                _ => (),
            });

        c.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }
}

impl<'a> Button<'a> {
    pub fn new(text: &str, context: Arc<Mutex<Context>>) -> Self {
        let mut vertices = Vec::new();
        let color: [f32; 3] = RED.into();
        let screen_size = {
            let c = context.lock().unwrap();
            (c.config.width as f32, c.config.height as f32)
        };
        let position = Vec2::default();

        let label = Text::new(
            context.clone(),
            text,
            ASSETS.get_font("Roboto.ttf").unwrap(),
            30.,
        );
        let label_bounds = label.bounds();

        vertices.push(Vertex {
            position: pixels_to_clip(position.x, position.y, screen_size.0, screen_size.1),
            color,
            tex_coords: [-1.0, -1.0],
        });
        vertices.push(Vertex {
            position: pixels_to_clip(
                position.x,
                position.y + label_bounds.height,
                screen_size.0,
                screen_size.1,
            ),
            color,
            tex_coords: [-1.0, -1.0],
        });
        vertices.push(Vertex {
            position: pixels_to_clip(
                position.x + label_bounds.width,
                position.y + label_bounds.height,
                screen_size.0,
                screen_size.1,
            ),
            color,
            tex_coords: [-1.0, -1.0],
        });
        vertices.push(Vertex {
            position: pixels_to_clip(
                position.x + label_bounds.width,
                position.y,
                screen_size.0,
                screen_size.1,
            ),
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
            vertices,
            context,
            position,
            size: (label_bounds.width, label_bounds.height).into(),
            vertex_buffer,
            label,
            state: ButtonState::None,
            mp: (0., 0.).into(),
            paddings: (0., 0., 0., 0.).into(),
        }
    }

    fn click(&mut self) {
        // println!("Click event");
        self.set_position(Vec2 { x: 100., y: 100. });
    }

    fn set_fill_color(&mut self, color: [f32; 3]) {
        let c = self.context.lock().unwrap();

        self.vertices.iter_mut().for_each(|v| v.color = color);
        c.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    pub fn set_paddings(&mut self, paddings: Vec4) {
        self.paddings = paddings;
    }
}

impl<'a> Widget for Button<'a> {
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
                self.mp = (x.round(), y.round()).into();
                println!("{}", self.mp);

                if inside(self.mp.x, self.mp.y) {
                    self.set_fill_color(GREEN.into());
                } else {
                    self.set_fill_color(RED.into());
                }
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                if inside(self.mp.x, self.mp.y) {
                    match *state {
                        ElementState::Pressed => {
                            self.click();
                            self.set_fill_color(BLUE.into());
                        }
                        ElementState::Released => {
                            self.set_fill_color(RED.into());
                        }
                    }
                }
            }
            _ => {}
        }

        self.state = s;
    }
}

impl<'a> Drawable for Button<'a> {
    fn draw<'b>(&'b mut self, render_pass: &mut RenderPass<'b>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..1);

        self.label.draw(render_pass);
        // self.label.draw(render_pass);
        // render_pass.set_pipeline(ctx.text_brush.render_pipeline());
    }
}

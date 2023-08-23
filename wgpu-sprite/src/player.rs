use std::{cell::RefCell, collections::HashMap, rc::Rc};

use wgpu::util::DeviceExt;
use winit::event::{VirtualKeyCode, WindowEvent};

use crate::{
    animation::Animation,
    assets::{AssetManager, T},
    math::Rect,
    Vertex,
};

const INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];
const FRAME_HEIGHT: f32 = 36.;
const FRAME_WIDTH: f32 = 33.;

pub struct Player {
    position: [f32; 2],
    speed: f32,
    vertex_buffer: wgpu::Buffer,
    vertices: [Vertex; 4],
    is_moving: bool,
    animations: HashMap<String, Animation>,
    current_animation: String,
    texture: T,
}

impl Player {
    pub fn new(asset_manager: Rc<RefCell<AssetManager>>, device: &wgpu::Device) -> Self {
        let vertices = [
            Vertex {
                position: [0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [0.0, -0.10666667],
                tex_coords: [0.0, 0.25],
            },
            Vertex {
                position: [0.08, -0.10666667],
                tex_coords: [0.25, 0.25],
            },
            Vertex {
                position: [0.08, 0.0],
                tex_coords: [0.25, 0.0],
            },
        ];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        asset_manager
            .borrow_mut()
            .load_texture("src/GR-panda.png", "sprite")
            .unwrap();
        let texture = (*asset_manager).borrow().get_texture("sprite").unwrap();

        let mut animations = HashMap::new();
        animations.insert(
            String::from("down"),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y: 0.,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 33.,
                        y: 0.,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 66.,
                        y: 0.,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 99.,
                        y: 0.,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                ],
                0.20,
            ),
        );
        let y = 36.;
        animations.insert(
            String::from("left"),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 33.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 66.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 99.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                ],
                0.20,
            ),
        );
        let y = 72.;
        animations.insert(
            String::from("right"),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 33.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 66.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 99.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                ],
                0.20,
            ),
        );
        let y = 108.;
        animations.insert(
            String::from("up"),
            Animation::new(
                &[
                    Rect {
                        x: 0.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 33.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 66.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                    Rect {
                        x: 99.,
                        y,
                        w: FRAME_WIDTH,
                        h: FRAME_HEIGHT,
                    },
                ],
                0.20,
            ),
        );

        Self {
            position: [0., 0.],
            speed: 0.001,
            vertices,
            vertex_buffer,
            is_moving: false,
            animations,
            current_animation: String::from("down"),
            texture,
        }
    }

    pub fn vertices(&self) -> &[Vertex; 4] {
        &self.vertices
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.is_moving {
            self.apply_animation();
            if let Some(animation) = self.animations.get_mut(&self.current_animation) {
                if animation.update(dt) {
                    return true;
                }
            }

            match self.current_animation.as_ref() {
                "up" => self.position = [self.position[0], self.position[1] + 1. * self.speed],
                "left" => self.position = [self.position[0] + -1. * self.speed, self.position[1]],
                "down" => self.position = [self.position[0], self.position[1] + -1. * self.speed],
                _ => self.position = [self.position[0] + 1. * self.speed, self.position[1]],
            }

            // println!("{:?}", self.position);

            self.update_position();
        }

        false
    }

    pub fn process_events(&mut self, event: &WindowEvent) {
        if let WindowEvent::KeyboardInput { input, .. } = event {
            let mut is_moving = false;

            if input.state == winit::event::ElementState::Pressed {
                if let Some(keycode) = input.virtual_keycode {
                    let mut animation = self.current_animation.clone();

                    match keycode {
                        VirtualKeyCode::Up => {
                            animation = String::from("up");
                            is_moving = true;
                        }
                        VirtualKeyCode::Left => {
                            animation = String::from("left");
                            is_moving = true;
                        }
                        VirtualKeyCode::Down => {
                            animation = String::from("down");
                            is_moving = true;
                        }
                        VirtualKeyCode::Right => {
                            animation = String::from("right");
                            is_moving = true;
                        }
                        _ => (),
                    }

                    if animation != self.current_animation {
                        if let Some(animation) = self.animations.get_mut(&self.current_animation) {
                            animation.reset();
                            self.apply_animation();
                        }

                        self.current_animation = animation;
                    }
                }
            }

            if !is_moving {
                if let Some(animation) = self.animations.get_mut(&self.current_animation) {
                    animation.reset();
                    self.apply_animation();
                }
            }

            self.is_moving = is_moving;
        }
    }

    fn update_position(&mut self) {
        self.vertices = [
            Vertex {
                position: [self.position[0] + 0.0, self.position[1] + 0.0],
                tex_coords: self.vertices[0].tex_coords,
            },
            Vertex {
                position: [self.position[0] + 0.0, self.position[1] + -0.10666667],
                tex_coords: self.vertices[1].tex_coords,
            },
            Vertex {
                position: [self.position[0] + 0.08, self.position[1] + -0.10666667],
                tex_coords: self.vertices[2].tex_coords,
            },
            Vertex {
                position: [self.position[0] + 0.08, self.position[1] + 0.0],
                tex_coords: self.vertices[3].tex_coords,
            },
        ];
    }

    fn apply_animation(&mut self) {
        if let Some(animation) = self.animations.get_mut(&self.current_animation) {
            let frame = animation.get_frame().unwrap();
            let width = 132.;
            let height = 144.;

            self.vertices = [
                Vertex {
                    position: [self.position[0] + 0.0, self.position[1] + 0.0],
                    tex_coords: [frame.x / width, frame.y / height],
                },
                Vertex {
                    position: [self.position[0] + 0.0, self.position[1] + -0.10666667],
                    tex_coords: [frame.x / width, frame.y / height + frame.h / height],
                },
                Vertex {
                    position: [self.position[0] + 0.08, self.position[1] + -0.10666667],
                    tex_coords: [
                        frame.x / width + frame.w / width,
                        frame.y / height + frame.h / height,
                    ],
                },
                Vertex {
                    position: [self.position[0] + 0.08, self.position[1] + 0.0],
                    tex_coords: [frame.x / width + frame.w / width, frame.y / height],
                },
            ];
        }
    }

    pub fn draw<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
        if let Some(animation) = self.animations.get(&self.current_animation) {
            render_pass.set_bind_group(0, &(*self.texture).1, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw_indexed(0..INDICES.len() as _, 0, 0..1); // 3.
        }
    }
}

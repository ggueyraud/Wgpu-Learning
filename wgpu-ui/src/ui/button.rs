use super::Widget;
use crate::graphics::shape::{RectangleShape, Shape};
use crate::graphics::text::Text;
use crate::graphics::{Drawable, Transformable, BLUE, GREEN, RED};
use crate::Ctx;
use crate::ASSETS;
use glam::{Vec2, Vec4};
use wgpu::RenderPass;
use winit::event::{ElementState, MouseButton, WindowEvent};

enum ButtonState {
    None,
    Hover,
    Click,
}

pub struct Button<'a> {
    rect: RectangleShape,
    // context: Ctx,
    label: Text<'a>,
    position: Vec2,
    state: ButtonState,
    mp: Vec2,
    paddings: Vec4,
}

impl<'a> Transformable for Button<'a> {
    fn position(&self) -> &Vec2 {
        self.rect.position()
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.label.set_position(position);
        self.rect.set_position(position);

        self.update(0.);
    }
}

impl<'a> Button<'a> {
    pub fn new(text: &str, context: Ctx) -> Button<'a> {
        let position = Vec2::default();

        let label = Text::new(
            context.clone(),
            text,
            ASSETS.get_font("Roboto.ttf").unwrap(),
            30.,
        );
        let label_bounds = label.bounds();

        let mut rect = RectangleShape::new(
            context.clone(),
            (label_bounds.width, label_bounds.height).into(),
        );
        rect.set_position(position);

        Self {
            rect,
            // context,
            position,
            label,
            state: ButtonState::None,
            mp: (0., 0.).into(),
            paddings: (0., 0., 0., 0.).into(),
        }
    }

    fn click(&mut self) {
        // println!("Click event");
        // self.set_position(Vec2 { x: 100., y: 100. });
        self.paddings.w += 5.;
        self.set_paddings(self.paddings);
    }

    pub fn set_paddings(&mut self, paddings: Vec4) {
        self.paddings = paddings;

        self.update(0.);
    }
}

impl<'a> Widget for Button<'a> {
    fn update(&mut self, _dt: f32) {
        // Calculate paddings
        let label_bounds = self.label.bounds();
        let size = Vec2 {
            x: label_bounds.width + self.paddings.x + self.paddings.w,
            y: label_bounds.height + self.paddings.y + self.paddings.z,
        };
        self.rect.set_size(size);

        let label_position = Vec2 {
            x: self.position.x + (size.x - label_bounds.width) / 2.,
            y: self.position.y + (size.y - label_bounds.height) / 2.,
        };
        self.label.set_position(label_position);
    }

    fn process_events(&mut self, event: &WindowEvent) {
        let mut s = ButtonState::None;
        let bounds = self.rect.bounds();
        let inside = |x: f32, y: f32| -> bool {
            x >= self.position.x
                && x <= self.position.x + bounds.width
                && y >= self.position.y
                && y <= self.position.y + bounds.height
        };

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = (position.x as f32, position.y as f32);
                self.mp = (x.round(), y.round()).into();

                if inside(self.mp.x, self.mp.y) {
                    self.rect.set_fill_color(GREEN);
                } else {
                    self.rect.set_fill_color(RED);
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
                            self.rect.set_fill_color(BLUE);
                        }
                        ElementState::Released => {
                            self.rect.set_fill_color(RED);
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
        self.rect.draw(render_pass);

        self.label.draw(render_pass);
    }
}

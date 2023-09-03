use super::{Widget, WidgetEvent};
use crate::graphics::shape::{RectangleShape, Shape};
use crate::graphics::text::Text;
use crate::graphics::{
    color::{BLUE, GREEN, RED},
    Drawable, Transformable,
};
use crate::Ctx;
use crate::ASSETS;
use glam::{Vec2, Vec4};
use wgpu::RenderPass;
use winit::event::{ElementState, MouseButton, WindowEvent};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ButtonEvent {
    Click,
    Hover,
}

impl WidgetEvent for ButtonEvent {}

pub struct Button<'a> {
    rect: RectangleShape,
    label: Text<'a>,
    position: Vec2,
    mouse_position: Vec2,
    paddings: Vec4,
    events: Vec<ButtonEvent>,
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
            position,
            label,
            mouse_position: Default::default(),
            paddings: (0., 0., 0., 0.).into(),
            events: Vec::new(),
        }
    }

    pub fn set_character_size(&mut self, character_size: f32) {
        self.label.set_character_size(character_size);
    }

    pub fn set_paddings(&mut self, paddings: Vec4) {
        self.paddings = paddings;

        self.update(0.);
    }

    pub fn size(&self) -> &Vec2 {
        self.rect.size()
    }
}

impl<'a> Widget for Button<'a> {
    // fn events(&mut self) -> std::vec::Drain<Box<dyn WidgetEvent>> {
    //     self.events.drain(..)
    // }

    fn emitted(&mut self, event: u32) -> bool {
        !self
            .events
            .drain(..)
            .filter(|e| *e as u32 == event)
            .collect::<Vec<_>>()
            .is_empty()
        // false
    }

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
        let bounds = self.rect.bounds();

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let (x, y) = (position.x as f32, position.y as f32);
                self.mouse_position = (x.round(), y.round()).into();

                if bounds.contains(self.mouse_position) {
                    self.rect.set_fill_color(GREEN);
                    self.events.push(ButtonEvent::Hover);
                } else {
                    self.rect.set_fill_color(RED);
                }
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => match state {
                ElementState::Pressed => {
                    if bounds.contains(self.mouse_position) {
                        match *state {
                            ElementState::Pressed => {
                                self.events.push(ButtonEvent::Click);
                                self.rect.set_fill_color(BLUE);
                            }
                            ElementState::Released => {
                                self.rect.set_fill_color(RED);
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

impl<'a> Drawable for Button<'a> {
    fn draw<'b>(&'b mut self, render_pass: &mut RenderPass<'b>) {
        self.rect.draw(render_pass);

        self.label.draw(render_pass);
    }
}

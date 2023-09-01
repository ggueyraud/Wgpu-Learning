use glam::Vec2;
use winit::event::{ElementState, MouseButton, WindowEvent};

use super::button::{Button, ButtonState};
use super::Widget;
use crate::graphics::shape::{RectangleShape, Shape};
use crate::graphics::text::Text;
use crate::graphics::{Drawable, Transformable, BLUE, RED};
use crate::{Ctx, ASSETS};

pub struct Window<'a> {
    title: Text<'a>,
    titlebar: RectangleShape,
    body: RectangleShape,
    mouse_position: Vec2,
    click_position: Option<Vec2>,
    visible: bool,
    close_btn: Button<'a>,
}

impl<'a> Window<'a> {
    pub fn new(context: Ctx, title: &str) -> Self {
        let mut titlebar = RectangleShape::new(context.clone(), (150., 20.).into());
        titlebar.set_fill_color(BLUE);

        let mut body = RectangleShape::new(context.clone(), (150., 150.).into());
        body.set_fill_color(RED);
        body.set_position(Vec2 {
            x: 0.,
            y: titlebar.bounds().height,
        });

        let mut close_btn = Button::new("x", context.clone());
        close_btn.set_character_size(16.);

        let mut window = Self {
            titlebar,
            body,
            title: Text::new(context, title, ASSETS.get_font("Roboto.ttf").unwrap(), 16.),
            mouse_position: Default::default(),
            click_position: None,
            visible: true,
            close_btn,
        };

        window.set_position((0., 0.).into());

        window
    }

    pub fn set_visibility(&mut self, visibility: bool) {
        self.visible = visibility;
    }
}

impl<'a> Widget for Window<'a> {
    fn process_events(&mut self, event: &winit::event::WindowEvent) {
        // Prevent events handling if widget is not displayed
        if !self.visible {
            return;
        }

        self.close_btn.process_events(event);

        if self.close_btn.state() == &ButtonState::Click {
            self.set_visibility(false);
        }

        match event {
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        let titlebar_bounds = self.titlebar.bounds();
                        // Check if click inside titlebar, if so, capture the click position
                        if titlebar_bounds.contains(self.mouse_position) {
                            println!("Click detected on titlebar");
                            // Calculate difference between mouse position and titlebar position
                            self.click_position = Some(
                                (
                                    self.mouse_position.x - titlebar_bounds.x,
                                    self.mouse_position.y - titlebar_bounds.y,
                                )
                                    .into(),
                            );
                        }
                    }
                    _ => {
                        self.click_position = None;
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = (position.x as f32, position.y as f32).into();

                if let Some(click_position) = self.click_position {
                    self.set_position(
                        (
                            position.x as f32 - click_position.x,
                            position.y as f32 - click_position.y,
                        )
                            .into(),
                    );
                }
            }
            _ => (),
        }
    }
}

impl<'a> Transformable for Window<'a> {
    fn position(&self) -> &glam::Vec2 {
        self.title.position()
    }

    fn set_position(&mut self, position: glam::Vec2) {
        self.titlebar.set_position(position);

        // Calculate title position
        self.title.set_position(Vec2 {
            x: position.x + 5.,
            y: position.y + (self.titlebar.bounds().height - self.title.bounds().height) / 2.,
        });

        let close_btn_size = self.close_btn.size();
        self.close_btn.set_position(Vec2 {
            x: position.x + self.titlebar.bounds().width - close_btn_size.x - 5.,
            y: position.y + (self.titlebar.bounds().height - close_btn_size.y) / 2.,
        });

        self.body.set_position(
            position
                + Vec2 {
                    x: 0.,
                    y: self.titlebar.bounds().height,
                },
        );
    }
}

impl<'a> Drawable for Window<'a> {
    fn draw<'b>(&'b mut self, render_pass: &mut wgpu::RenderPass<'b>) {
        if self.visible {
            self.titlebar.draw(render_pass);
            self.body.draw(render_pass);
            self.title.draw(render_pass);
            self.close_btn.draw(render_pass);
        }
    }
}

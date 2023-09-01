use glam::Vec2;

use super::Widget;
use crate::graphics::shape::{RectangleShape, Shape};
use crate::graphics::text::Text;
use crate::graphics::{Drawable, Transformable, BLUE, RED};
use crate::{Ctx, ASSETS};

pub struct Window<'a> {
    title: Text<'a>,
    titlebar: RectangleShape,
    body: RectangleShape,
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

        let mut window = Self {
            titlebar,
            body,
            title: Text::new(context, title, ASSETS.get_font("Roboto.ttf").unwrap(), 16.),
        };

        window.set_position((0., 0.).into());

        window
    }
}

impl<'a> Widget for Window<'a> {
    fn process_events(&mut self, _event: &winit::event::WindowEvent) {}
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
        self.titlebar.draw(render_pass);
        self.body.draw(render_pass);
        self.title.draw(render_pass);
    }
}

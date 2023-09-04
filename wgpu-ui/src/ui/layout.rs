use std::{collections::HashMap, ops::Deref};

use glam::Vec2;

use super::{Widget, WidgetId};
use crate::graphics::{Drawable, Transformable};

#[derive(Debug)]
pub enum Direction {
    Vertical,
    Horizontal,
}

pub struct Layout {
    direction: Direction,
    widgets: HashMap<WidgetId, Box<dyn Widget>>,
    position: Vec2,
    visible: bool,
    size: Vec2,
    spacing: f32,
    counter: u16,
}

impl Layout {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            widgets: HashMap::new(),
            position: Default::default(),
            visible: true,
            size: Default::default(),
            spacing: 3.,
            counter: 0,
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.counter += 1;
        self.widgets.insert(self.counter, widget);

        self.update();
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn set_spacing(&mut self, spacing: f32) {
        self.spacing = spacing;

        self.update();
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;

        self.update();
    }
}

impl Transformable for Layout {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;

        self.update();
    }
}

impl Drawable for Layout {
    fn draw<'b>(&'b mut self, render_pass: &mut wgpu::RenderPass<'b>) {
        self.widgets
            .iter_mut()
            .for_each(|(_, widget)| widget.draw(render_pass));
    }
}

impl Widget for Layout {
    fn process_events(&mut self, event: &winit::event::WindowEvent) {
        self.widgets
            .iter_mut()
            .for_each(|(_, widget)| widget.process_events(event));
    }

    fn update(&mut self) {
        let mut biggest_dimensions = Vec2::default();

        self.widgets.iter().for_each(|(_, widget)| {
            let size = widget.size();

            if size.x > biggest_dimensions.x {
                biggest_dimensions.x = size.x;
            }

            if size.y > biggest_dimensions.y {
                biggest_dimensions.y = size.y;
            }
        });

        self.widgets
            .iter_mut()
            .enumerate()
            .for_each(|(i, (_, widget))| {
                let position = match self.direction {
                    Direction::Horizontal => Vec2 {
                        x: (biggest_dimensions.x + self.spacing) * i as f32,
                        y: self.spacing,
                    },
                    _ => Vec2 {
                        x: self.spacing,
                        y: (biggest_dimensions.y + self.spacing) * i as f32,
                    },
                } + self.position;

                widget.set_position(position);
            });

        self.widgets
            .iter_mut()
            .for_each(|(_, widget)| widget.update());
    }

    fn set_visibility(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn size(&self) -> &Vec2 {
        &self.size
    }
}

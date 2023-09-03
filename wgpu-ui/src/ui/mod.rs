use std::collections::HashMap;

use crate::graphics::{Drawable, Transformable};
use wgpu::RenderPass;
use winit::event::WindowEvent;

pub mod button;
pub mod window;

pub type WidgetId = i16;

pub struct Ui {
    widgets: HashMap<WidgetId, Box<dyn Widget>>,
    counter: i16,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            widgets: HashMap::new(),
            counter: 0,
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget>) -> WidgetId {
        self.counter += 1;
        self.widgets.insert(self.counter, widget);

        self.counter
    }

    pub fn get(&mut self, id: WidgetId) -> Option<&mut Box<dyn Widget>> {
        self.widgets.get_mut(&id)
    }

    pub fn process_events(&mut self, event: &WindowEvent) {
        self.widgets
            .iter_mut()
            .for_each(|(_, w)| w.process_events(event));
    }

    pub fn update(&mut self, dt: f32) {
        self.widgets.iter_mut().for_each(|(_, w)| w.update(dt));
    }

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        render_pipeline: &'a wgpu::RenderPipeline,
    ) {
        self.widgets.iter_mut().for_each(|(_, w)| {
            render_pass.set_pipeline(render_pipeline);
            w.draw(render_pass);
        });
    }
}

pub trait WidgetEvent {}

pub trait Widget: Drawable + Transformable {
    fn process_events(&mut self, event: &WindowEvent);

    fn events(&mut self, event_handler: Box<dyn Fn(u32)>);

    fn emitted(&mut self, event: u32) -> bool;

    fn update(&mut self, _dt: f32) {}

    fn set_visibility(&mut self, visible: bool);
    fn visible(&self) -> bool;
}

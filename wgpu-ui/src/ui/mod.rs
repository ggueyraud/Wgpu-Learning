use crate::graphics::{Drawable, Transformable};
use wgpu::RenderPass;
use winit::event::WindowEvent;

pub mod button;
pub mod window;

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

    pub fn draw<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        render_pipeline: &'a wgpu::RenderPipeline,
    ) {
        self.widgets.iter_mut().for_each(|w| {
            render_pass.set_pipeline(render_pipeline);
            w.draw(render_pass);
        });
    }
}

pub trait Widget: Drawable + Transformable {
    fn process_events(&mut self, event: &WindowEvent);

    fn update(&mut self, _dt: f32) {}
}

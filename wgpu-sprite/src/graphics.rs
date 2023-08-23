pub trait Drawable {
    fn draw<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>);
}

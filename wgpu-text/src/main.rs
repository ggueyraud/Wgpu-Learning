use wgpu_text::run;

fn main() {
    tracing_subscriber::fmt::init();

    pollster::block_on(run());
}

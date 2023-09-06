use assets::Assets;
use graphics::{text::TextBrush, Transformable, Vertex};
use once_cell::sync::{Lazy, OnceCell};
use std::{
    cell::RefCell,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
    collections::HashMap,
};
use ui::{
    button::{Button, ButtonEvent},
    layout::Layout,
    Ui, Widget, WidgetId,
};
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod assets;
mod graphics;
mod math;
mod ui;

const INDICES: &[u16] = &[0, 1, 3, 1, 2, 3];

static PIPELINES: OnceCell<HashMap<String, (wgpu::RenderPipeline, Option<wgpu::BindGroupLayout>)>> = OnceCell::new();
static TEXT_BRUSH: OnceCell<TextBrush> = OnceCell::new();
static ASSETS: Lazy<Assets> = Lazy::new(|| {
    let mut assets = Assets::new();
    let _ = assets.load_font(Path::new("assets/Roboto.ttf"));

    assets
});

#[derive(Debug)]
pub struct Context {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

pub type Ctx = Arc<Mutex<Context>>;

struct State {
    context: Arc<Mutex<Context>>,
    surface: wgpu::Surface,
    index_buffer: wgpu::Buffer,
    ui: Ui,
    // btn_id: WidgetId,
    // window_id: WidgetId,
}

impl State {
    async fn new(window: &Window) -> State {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let text_brush = TextBrush::new(&device, config.format);

        let _ = TEXT_BRUSH.set(text_brush);

        let mut render_pipelines = HashMap::new();
        render_pipelines.insert("std".to_string(), (render_pipeline, None));
        let _ = PIPELINES.set(render_pipelines);

        let context = Arc::new(Mutex::new(Context {
            config,
            device,
            queue
        }));

        let mut ui = Ui::new();
        // let mut btn = Button::new("Lorem ipsum", context.clone());
        // btn.set_position(glam::Vec2 { x: 0., y: 200. });
        // btn.set_paddings((10., 20., 20., 10.).into());
        // btn.events(Box::new(|event| {
        //     let v = ButtonEvent::Click as u32;
        //     if let v = event {}
        // }));
        // let btn_id = ui.add(Box::new(btn));

        // let mut window = ui::window::Window::new(context.clone(), "Lorem ipsum");
        // window.set_position((100., 50.).into());
        // let window_id = ui.add(Box::new(window));

        let mut layout = Layout::new(ui::layout::Direction::Vertical);
        layout.set_position((100., 100.).into());
        layout.set_spacing(20.);
        layout.add_widget(Box::new(Button::new("Lorem ipsum", context.clone())));
        layout.add_widget(Box::new(Button::new("dolor sit amet", context.clone())));
        layout.add_widget(Box::new(Button::new("dolor sit amet", context.clone())));
        layout.add_widget(Box::new(Button::new("dolor sit amet", context.clone())));
        ui.add(Box::new(layout));

        Self {
            surface,
            index_buffer,
            ui,
            context,
            // btn_id,
            // window_id,
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.ui.process_events(event);

        // let visible = Rc::new(RefCell::new(false));

        // if let Some(btn) = self.ui.get(self.btn_id) {
        //     if btn.emitted(ButtonEvent::Click as u32) {
        //         (*visible.borrow_mut()) = true;
        //     }
        // }

        // if let Some(window) = self.ui.get(self.window_id) {
        //     if !window.visible() && *visible.borrow() {
        //         window.set_visibility(*visible.borrow());
        //     }
        // }

        false
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            let mut context = self.context.lock().unwrap();
            context.config.width = new_size.width;
            context.config.height = new_size.height;
            self.surface.configure(&context.device, &context.config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let context = self.context.lock().unwrap();

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        drop(context);

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            // self.ui.draw(&mut render_pass, &self.render_pipeline);
            self.ui.draw(&mut render_pass);
        }

        let context = self.context.lock().unwrap();
        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Wgpu Basic UI");
    let mut state = State::new(&window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => {
                    let (width, height) = {
                        let context = state.context.lock().unwrap();

                        (context.config.width, context.config.height)
                    };

                    state.resize((width, height).into())
                }
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            window.request_redraw();
        }
        _ => {}
    });
}

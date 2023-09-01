use crate::{
    math::{pixels_to_clip, Rect},
    Ctx, TEXT_BRUSH,
};

use super::{Drawable, Transformable, Vertex};
use glam::Vec2;
use rusttype::{gpu_cache::Cache, point, PositionedGlyph, Scale};
use wgpu::util::DeviceExt;

const TEXTURE_WIDTH: u32 = 512;
const TEXTURE_HEIGHT: u32 = 512;

fn layout_paragraph<'a>(
    font: &rusttype::Font<'a>,
    scale: Scale,
    width: u32,
    text: &str,
) -> (Vec<PositionedGlyph<'a>>, Rect) {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
    let mut bounds = Rect::default();

    for c in text.chars() {
        if c.is_control() {
            match c {
                '\r' => {
                    caret = point(0.0, caret.y + advance_height);
                }
                '\n' => {}
                _ => {}
            }
            continue;
        }
        let base_glyph = font.glyph(c);
        if let Some(id) = last_glyph_id.take() {
            caret.x += font.pair_kerning(scale, id, base_glyph.id());
        }
        last_glyph_id = Some(base_glyph.id());
        let mut glyph = base_glyph.scaled(scale).positioned(caret);

        if let Some(bb) = glyph.pixel_bounding_box() {
            bounds.height = bounds.height.max(bb.max.y as f32);

            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        bounds.width += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }

    (result, bounds)
}

fn generate_vertices(
    queue: &wgpu::Queue,
    texture: &wgpu::Texture,
    font: &rusttype::Font,
    text: &str,
    character_size: f32,
    position: Vec2,
    size: (f32, f32),
) -> (Vec<Vertex>, Rect) {
    let (width, height) = (TEXTURE_WIDTH, TEXTURE_HEIGHT);
    let mut cache = Cache::builder().dimensions(width, height).build();
    let (glyphs, mut bounds) = layout_paragraph(
        font,
        Scale::uniform(character_size * 1.),
        size.0 as u32,
        text,
    );
    bounds.x = position.x;
    bounds.y = position.y;

    for glyph in &glyphs {
        cache.queue_glyph(0, glyph.clone());
    }

    cache
        .cache_queued(|rect, data| {
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: rect.min.x,
                        y: rect.min.y,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(rect.width()),
                    rows_per_image: Some(rect.height()),
                },
                wgpu::Extent3d {
                    width: rect.width(),
                    height: rect.height(),
                    depth_or_array_layers: 1,
                },
            );
        })
        .unwrap();

    let color = [1.0, 1.0, 1.0];
    // let origin = pixels_to_clip(position.x, position.y, size.0, size.1);
    // let origin = point(origin[0], origin[1]);
    let origin = point(0., 0.);

    let vertices = glyphs
        .iter()
        .filter_map(|g| cache.rect_for(0, g).ok().flatten())
        .flat_map(|(uv_rect, screen_rect)| {
            // TODO : refactor this calculation
            let a = pixels_to_clip(
                position.x + screen_rect.min.x as f32,
                position.y + screen_rect.min.y as f32,
                size.0,
                size.1,
            );
            let b = pixels_to_clip(
                position.x + screen_rect.max.x as f32,
                position.y + screen_rect.max.y as f32,
                size.0,
                size.1,
            );
            let gl_rect = rusttype::Rect {
                min: // origin
                    // This convert into screen coordinate
                    point(a[0], a[1]),
                    //  (vector(
                    //     screen_rect.min.x as f32 / size.0 as f32 - 0.5,
                    //     1.0 - screen_rect.min.y as f32 / size.1 as f32 - 0.5,
                    // )) * 2.0,
                max: // origin
                    point(b[0], b[1]),
                    // + (vector(
                    //     screen_rect.max.x as f32 / size.0 as f32 - 0.5,
                    //     1.0 - screen_rect.max.y as f32 / size.1 as f32 - 0.5,
                    // )) * 2.0,
            };

            vec![
                Vertex {
                    position: [gl_rect.min.x, gl_rect.max.y],
                    tex_coords: [uv_rect.min.x, uv_rect.max.y],
                    color,
                },
                Vertex {
                    position: [gl_rect.min.x, gl_rect.min.y],
                    tex_coords: [uv_rect.min.x, uv_rect.min.y],
                    color,
                },
                Vertex {
                    position: [gl_rect.max.x, gl_rect.min.y],
                    tex_coords: [uv_rect.max.x, uv_rect.min.y],
                    color,
                },
                Vertex {
                    position: [gl_rect.max.x, gl_rect.min.y],
                    tex_coords: [uv_rect.max.x, uv_rect.min.y],
                    color,
                },
                Vertex {
                    position: [gl_rect.max.x, gl_rect.max.y],
                    tex_coords: [uv_rect.max.x, uv_rect.max.y],
                    color,
                },
                Vertex {
                    position: [gl_rect.min.x, gl_rect.max.y],
                    tex_coords: [uv_rect.min.x, uv_rect.max.y],
                    color,
                },
            ]
        })
        .collect();

    (vertices, bounds)
}

pub struct Text<'a> {
    context: Ctx,
    text: String,
    character_size: f32,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    bind_group: wgpu::BindGroup,
    position: Vec2,
    geometry_need_update: bool,
    vertices: Vec<Vertex>,
    texture: wgpu::Texture,
    font: &'a rusttype::Font<'a>,
    bounds: Rect,
}

impl<'a> Text<'a> {
    pub fn new(context: Ctx, text: &str, font: &'a rusttype::Font, character_size: f32) -> Text<'a> {
        let c = context.lock().unwrap();

        let texture_size = wgpu::Extent3d {
            width: TEXTURE_WIDTH,
            height: TEXTURE_HEIGHT,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = c.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: Some("Diffuse texture"),
            view_formats: &[],
        });
        let diffuse_texture_view =
            diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = c.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let (vertices, bounds) = generate_vertices(
            &c.queue,
            &diffuse_texture,
            font,
            text,
            character_size,
            Vec2::default(),
            (c.config.width as f32, c.config.height as f32),
        );

        let bind_group = c.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &TEXT_BRUSH.get().unwrap().bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let vertex_buffer = c
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            });

        Self {
            text: text.to_string(),
            character_size,
            vertex_buffer,
            num_vertices: vertices.len() as _,
            bind_group,
            position: Vec2::default(),
            geometry_need_update: false,
            vertices: Vec::new(),
            texture: diffuse_texture,
            font,
            context: context.clone(),
            bounds,
        }
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }

    fn ensure_geometry_update(&mut self) {
        if !self.geometry_need_update {
            return;
        }

        self.geometry_need_update = false;
        self.vertices.clear();
        self.bounds = Rect::default();

        let ctx = self.context.lock().unwrap();

        let (vertices, bounds) = generate_vertices(
            &ctx.queue,
            &self.texture,
            self.font,
            &self.text,
            self.character_size,
            self.position,
            (ctx.config.width as f32, ctx.config.height as f32),
        );
        self.vertices = vertices;
        self.bounds = bounds;
        self.num_vertices = self.vertices.len() as _;

        ctx.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    pub fn set_character_size(&mut self, character_size: f32) {
        self.character_size = character_size;
        self.geometry_need_update = true;

        self.ensure_geometry_update();
    }
}

impl<'a> Drawable for Text<'a> {
    fn draw<'b>(&'b mut self, render_pass: &mut wgpu::RenderPass<'b>) {
        // TODO : call ensure_geometry_upate

        render_pass.set_pipeline(TEXT_BRUSH.get().unwrap().render_pipeline());

        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}

impl<'a> Transformable for Text<'a> {
    fn position(&self) -> &Vec2 {
        &self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
        self.geometry_need_update = true;

        // TODO : in future this should not be manually call after data alteration
        self.ensure_geometry_update();
    }
}

pub struct TextBrush {
    bind_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
}

impl TextBrush {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/text.wgsl"));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render pipeline layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text render pipeline"),
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
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                // cull_mode: Some(wgpu::Face::Back),
                cull_mode: None,
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

        Self {
            render_pipeline,
            bind_group_layout,
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
    }
}

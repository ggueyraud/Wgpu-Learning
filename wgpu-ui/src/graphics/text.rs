use std::sync::{Arc, Mutex};

use crate::Context;

use super::{font::Font, Drawable, Vertex, Transformable};
use glam::Vec2;
use rusttype::{gpu_cache::Cache, point, vector, PositionedGlyph, Rect, Scale};
use wgpu::util::DeviceExt;

fn layout_paragraph<'a>(
    font: &rusttype::Font<'a>,
    scale: Scale,
    width: u32,
    text: &str,
) -> Vec<PositionedGlyph<'a>> {
    let mut result = Vec::new();
    let v_metrics = font.v_metrics(scale);
    let advance_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
    let mut caret = point(0.0, v_metrics.ascent);
    let mut last_glyph_id = None;
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
            if bb.max.x > width as i32 {
                caret = point(0.0, caret.y + advance_height);
                glyph.set_position(caret);
                last_glyph_id = None;
            }
        }
        caret.x += glyph.unpositioned().h_metrics().advance_width;
        result.push(glyph);
    }
    result
}

pub struct Text {
    transformable: Transformable,
    text: String,
    character_size: u8,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    bind_group: wgpu::BindGroup,
}

impl Text {
    pub fn new(
        context: Arc<Mutex<Context>>,
        text: &str,
        font: &Font,
        character_size: f32,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let c = context.lock().unwrap();

        let scale: f32 = 1.; // TODO : remplace with window scale factor

        let (width, height) = (512, 512);
        let mut cache = Cache::builder().dimensions(width, height).build();
        let texture_size = wgpu::Extent3d {
            width,
            height,
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

        let glyphs = layout_paragraph(
            &font.internal,
            Scale::uniform(character_size * scale),
            c.config.width,
            text,
        );

        for glyph in &glyphs {
            cache.queue_glyph(0, glyph.clone());
        }

        cache
            .cache_queued(|rect, data| {
                c.queue.write_texture(
                    wgpu::ImageCopyTexture {
                        texture: &diffuse_texture,
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

        let (vertex_buffer, num_vertices) = {
            let color = [1.0, 1.0, 1.0];
            let origin = point(0.0, 0.0);

            let vertices: Vec<Vertex> = glyphs
                .iter()
                .filter_map(|g| cache.rect_for(0, g).ok().flatten())
                .flat_map(|(uv_rect, screen_rect)| {
                    let gl_rect = Rect {
                        min: origin
                            + (vector(
                                screen_rect.min.x as f32 / c.config.width as f32 - 0.5,
                                1.0 - screen_rect.min.y as f32 / c.config.height as f32 - 0.5,
                            )) * 2.0,
                        max: origin
                            + (vector(
                                screen_rect.max.x as f32 / c.config.width as f32 - 0.5,
                                1.0 - screen_rect.max.y as f32 / c.config.height as f32 - 0.5,
                            )) * 2.0,
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

            (
                c.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    }),
                vertices.len() as u32,
            )
        };

        let bind_group = c.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
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

        Self {
            text: text.to_string(),
            character_size: 30,
            vertex_buffer,
            num_vertices,
            bind_group,
            transformable: Transformable::new()
        }
    }
}

impl Drawable for Text {
    fn draw<'b>(&'b self, render_pass: &mut wgpu::RenderPass<'b>) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}

pub struct TextBrush {
    bind_group_layout: wgpu::BindGroupLayout,
    render_pipeline: wgpu::RenderPipeline,
}

impl TextBrush {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader: wgpu::ShaderModule =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/text.wgsl"));

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

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.render_pipeline);
    }
}

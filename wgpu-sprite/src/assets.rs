use crate::texture::{Font, Texture};
use anyhow::*;
use std::{collections::HashMap, rc::Rc};

pub type T = Rc<(Texture, wgpu::BindGroup)>;
pub type FontRes = Rc<(Font, wgpu::BindGroup)>;

pub struct AssetManager {
    textures: HashMap<String, T>,
    fonts: HashMap<String, FontRes>,
    device: Rc<wgpu::Device>,
    queue: Rc<wgpu::Queue>,
    pub texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl AssetManager {
    pub fn new(device: Rc<wgpu::Device>, queue: Rc<wgpu::Queue>) -> Self {
        let texture_bind_group_layout =
            (*device).create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        Self {
            device,
            queue,
            textures: HashMap::new(),
            texture_bind_group_layout,
            fonts: HashMap::new(),
        }
    }

    // pub fn texture_bind

    pub fn get_texture(&self, name: &str) -> Option<T> {
        self.textures.get(name).cloned()
    }

    pub fn load_texture(&mut self, path: &str, name: &str) -> Result<()> {
        let img = image::open(path)?;
        let texture = Texture::from_image(&self.device, &self.queue, &img, Some(name))?;
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: Some(&format!("{name}_bind_group")),
        });

        self.textures
            .insert(name.to_owned(), Rc::new((texture, bind_group)));

        Ok(())
    }

    pub fn get_font(&self, name: &str) -> Option<FontRes> {
        self.fonts.get(name).cloned()
    }

    pub fn load_font(&mut self, path: &str, name: &str, scale_factor: f64) -> Result<()> {
        let font = Font::load_from_file(&path, &self.device, &self.queue, scale_factor, &name)?;
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&font.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&font.texture.sampler),
                },
            ],
            label: Some(&format!("{name}_bind_group")),
        });
        self.fonts
            .insert(name.to_owned(), Rc::new((font, bind_group)));

        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageError};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device, Extent3d,
    ImageCopyTexture, ImageDataLayout, Queue, Sampler, SamplerDescriptor, TextureDescriptor, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor,
};

pub struct Texture
{
    pub texture_size: Extent3d,
    pub texture: wgpu::Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub bind_group: BindGroup,
}

impl Texture
{
    const MIP_LEVEL_COUNT: u32 = 1;
    const SAMPLE_COUNT: u32 = 1;

    fn from_image(
        image: &DynamicImage,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> Result<Self, ImageError>
    {
        let rgba = image.to_rgba8();
        let dims = image.dimensions();

        let premultiplied: Vec<u8> = rgba
            .pixels()
            .flat_map(|p| {
                let a = p[3] as f32 / 255.0;
                [
                    (p[0] as f32 * a) as u8,
                    (p[1] as f32 * a) as u8,
                    (p[2] as f32 * a) as u8,
                    p[3],
                ]
            })
            .collect();

        let texture_size = Extent3d {
            width: dims.0,
            height: dims.1,
            depth_or_array_layers: 1,
        };

        let diffuse_texture = device.create_texture(&TextureDescriptor {
            label: Some("diffuse_texture"),
            size: texture_size,
            mip_level_count: Self::MIP_LEVEL_COUNT,
            sample_count: Self::SAMPLE_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &premultiplied,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * texture_size.width),
                rows_per_image: Some(texture_size.height),
            },
            texture_size,
        );

        let view = diffuse_texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("diffuse_bind_group"),
            layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok(Self {
            texture_size,
            texture: diffuse_texture,
            view,
            sampler,
            bind_group,
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_path(path: &Path, device: &Device, queue: &Queue, layout: &BindGroupLayout)
    -> Result<Self, ImageError>
    {
        let image = image::open(path)?;
        Self::from_image(&image, device, queue, layout)
    }

    pub fn from_bytes(
        bytes: &[u8],
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> Result<Self, ImageError>
    {
        let image = image::load_from_memory(bytes)?;
        Self::from_image(&image, device, queue, layout)
    }
}

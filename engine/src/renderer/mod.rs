use std::{collections::HashMap, sync::Arc};

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendComponent, BlendFactor,
    BlendOperation, BlendState, ColorWrites, CommandEncoderDescriptor, Device, FragmentState, IndexFormat,
    MultisampleState, Operations, PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderStages, TextureFormat, TextureView,
    VertexState, include_wgsl,
};

use crate::{
    jade::{camera::Camera, ecs::object::Object},
    renderer::{batch::TextureBatch, camera_uniform::CameraBuffer, mesh::Mesh, texture::Texture, vertex::Vertex},
    util::assets::assetpool::TextureAsset,
};

pub mod batch;
pub mod bufferpool;
pub mod camera_uniform;
pub mod mesh;
pub mod texture;
pub mod vertex;

pub type ZIndex = i32;

pub trait Renderable
{
    fn mesh(&self) -> Mesh;
    fn texture(&self) -> Option<&TextureAsset>;
    fn z_index(&self) -> ZIndex;
}

pub struct Renderer
{
    pub texture_bind_group_layout: BindGroupLayout,
    pub camera_buffer: CameraBuffer,
    pipeline: RenderPipeline,

    // TODO: right now this works because the assetpool ensures the pointers are constant,
    // however it feels slightly dodgy
    batches: HashMap<*const Texture, (TextureAsset, TextureBatch)>,
}

impl Renderer
{
    pub fn new(device: &Device, surface_format: TextureFormat) -> Self
    {
        let camera_buffer = CameraBuffer::new(device);

        let texture_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let shader = device.create_shader_module(include_wgsl!("../../assets/shaders/shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("render_pipeline_layout"),
            bind_group_layouts: &[&camera_buffer.layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::layout()],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(BlendState {
                        color: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                        alpha: BlendComponent {
                            src_factor: BlendFactor::One,
                            dst_factor: BlendFactor::OneMinusSrcAlpha,
                            operation: BlendOperation::Add,
                        },
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });

        Self {
            texture_bind_group_layout,
            camera_buffer,
            pipeline,
            batches: HashMap::new(),
        }
    }

    pub fn draw(&mut self, renderables: &[Object], device: &Device, queue: &Queue, view: &TextureView, camera: &Camera)
    {
        self.camera_buffer.update(queue, camera);

        for renderable in renderables
        {
            let mesh = renderable.mesh();
            let Some(texture) = renderable.texture()
            else
            {
                continue;
            };
            let z = renderable.z_index();

            // TODO: again this keying only works because assetpool. improve in future
            let key = Arc::as_ptr(texture);

            let indices: Vec<u32> = mesh.indices.iter().map(|&i| i as u32).collect();

            self.batches
                .entry(key)
                .or_insert_with(|| (Arc::clone(texture), TextureBatch::new(device)))
                .1
                .push(&mesh.vertices, &indices, z);
        }

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.camera_buffer.bind_group, &[]);

            for (texture, batch) in self.batches.values_mut()
            {
                let Some(slice) = batch.flush(queue)
                else
                {
                    continue;
                };

                pass.set_bind_group(1, &texture.bind_group, &[]);
                pass.set_vertex_buffer(0, batch.pool.vertex_buffer());
                pass.set_index_buffer(batch.pool.index_buffer(), IndexFormat::Uint32);
                pass.draw_indexed(0..slice.0, 0, 0..1);
            }
        }

        queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn remove_batch(&mut self, texture: &Texture) { self.batches.remove(&(texture as *const Texture)); }
}

use wgpu::{
    BindGroup, BindGroupLayout, BlendState, Buffer, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites,
    Device, FragmentState, IndexFormat, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology, Queue, RenderPass,
    RenderPipeline, RenderPipelineDescriptor, TextureFormat, VertexState, include_wgsl,
};

use crate::renderer::primitive::{draw_command::DrawCommand, vertex::ColoredVertex};

pub mod draw_command;
mod vertex;

pub struct PrimitivePipeline
{
    pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    max_vertices: usize,
    max_indices: usize,
}

impl PrimitivePipeline
{
    const MAX_VERTICES: usize = 64 * 1024;
    const MAX_INDICES: usize = 96 * 1024;

    pub fn new(device: &Device, surface_format: &TextureFormat, camera_layout: &BindGroupLayout) -> Self
    {
        let shader = device.create_shader_module(include_wgsl!("../../../assets/shaders/shader.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("primitive_pipeline_layout"),
            bind_group_layouts: &[Some(camera_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("primitive_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(ColoredVertex::layout())],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: *surface_format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("primitive_vertex_buffer"),
            size: (Self::MAX_VERTICES * size_of::<ColoredVertex>()) as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("primitive_index_buffer"),
            size: (Self::MAX_INDICES * size_of::<u32>()) as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            max_vertices: Self::MAX_VERTICES,
            max_indices: Self::MAX_INDICES,
        }
    }

    pub fn draw(&self, commands: &[DrawCommand], queue: &Queue, pass: &mut RenderPass, camera_bg: &BindGroup)
    {
        let (verts, indices) = commands.iter().fold(
            (Vec::<ColoredVertex>::new(), Vec::<u32>::new()),
            |(mut v, mut i), cmd| {
                cmd.tessellate(&mut v, &mut i);
                (v, i)
            },
        );

        if verts.is_empty()
        {
            return;
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&verts));
        queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&indices));

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, camera_bg, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);

        pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
    }
}

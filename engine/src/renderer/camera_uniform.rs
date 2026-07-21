use glam::Mat4;
use log::debug;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, Buffer, BufferBindingType, BufferUsages, Device, Queue, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::jade::camera::Camera;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform
{
    view_projection: [[f32; 4]; 4],
}

impl CameraUniform
{
    const IDENTITY: Self = Self::from_matrix(Mat4::IDENTITY);

    pub const fn from_matrix(matrix: Mat4) -> Self
    {
        Self {
            view_projection: matrix.to_cols_array_2d(),
        }
    }
}

pub struct CameraBuffer
{
    pub buffer: Buffer,
    pub bind_group: BindGroup,
    pub layout: BindGroupLayout,
}

impl CameraBuffer
{
    pub fn new(device: &Device) -> Self
    {
        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("camera_bind_group_layout"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("camera_buffer"),
            contents: bytemuck::bytes_of(&CameraUniform::IDENTITY),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("camera_bind_group"),
            layout: &layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            buffer,
            bind_group,
            layout,
        }
    }

    pub fn update(&self, queue: &Queue, camera: &Camera)
    {
        queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::bytes_of(&CameraUniform::from_matrix(camera.view_projection())),
        );
    }
}

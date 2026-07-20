use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device, Queue};

use crate::renderer::vertex::Vertex;

pub struct BufferPool
{
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    vertex_capacity: usize,
    index_capacity: usize,
    current_vertex_count: usize,
    current_index_count: usize,
}

impl BufferPool
{
    pub fn new(device: &Device, max_vertices: usize, max_indices: usize) -> Self
    {
        let vertex_capacity = max_vertices * size_of::<Vertex>();
        let index_capacity = max_indices * size_of::<u32>();

        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("persistent_vertex_buffer"),
            size: vertex_capacity as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("persistent_index_buffer"),
            size: index_capacity as u64,
            usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            vertex_buffer,
            index_buffer,
            vertex_capacity,
            index_capacity,
            current_vertex_count: 0,
            current_index_count: 0,
        }
    }

    pub fn write(&mut self, queue: &Queue, vertices: &[Vertex], indices: &[u32]) -> Option<BufferSlice>
    {
        if vertices.len() > self.vertex_capacity || indices.len() > self.index_capacity
        {
            log::warn!(
                "BufferPool overflow: {} vertices (cap {}), {} indices (cap {})",
                vertices.len(),
                self.vertex_capacity,
                indices.len(),
                self.index_capacity,
            );
            return None;
        }

        let vb = bytemuck::cast_slice(vertices);
        let ib = bytemuck::cast_slice(indices);

        queue.write_buffer(&self.vertex_buffer, 0, vb);
        queue.write_buffer(&self.index_buffer, 0, ib);

        self.current_vertex_count = vertices.len();
        self.current_index_count = indices.len();

        Some(BufferSlice(indices.len() as u32))
    }

    pub fn reset(&mut self)
    {
        self.current_vertex_count = 0;
        self.current_index_count = 0;
    }

    pub fn fits(&self, vertex_count: usize, index_count: usize) -> bool
    {
        vertex_count <= self.vertex_capacity && index_count <= self.index_capacity
    }

    pub fn vertex_buffer(&self) -> wgpu::BufferSlice<'_> { self.vertex_buffer.slice(..) }

    pub fn index_buffer(&self) -> wgpu::BufferSlice<'_> { self.index_buffer.slice(..) }
}

pub struct BufferSlice(pub u32);

use log::warn;
use wgpu::{Device, Queue};

use crate::renderer::{
    bufferpool::{BufferPool, BufferSlice},
    vertex::Vertex,
};

pub struct TextureBatch
{
    pub pool: BufferPool,
    pub min_z: i32,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl TextureBatch
{
    const MAX_VERTICES: usize = 8 * 1024;
    const MAX_INDICES: usize = 12 * 1024;

    pub fn new(device: &Device) -> Self
    {
        Self {
            pool: BufferPool::new(device, Self::MAX_VERTICES, Self::MAX_INDICES),
            min_z: i32::MAX,
            vertices: Vec::with_capacity(Self::MAX_VERTICES),
            indices: Vec::with_capacity(Self::MAX_INDICES),
        }
    }

    pub fn push(&mut self, vertices: &[Vertex], indices: &[u32], z: i32) -> bool
    {
        if !self
            .pool
            .fits(self.vertices.len() + vertices.len(), self.indices.len() + indices.len())
        {
            warn!("TextureBatch full, renderable dropped");
            return false;
        }

        let offset = self.vertices.len() as u32;
        self.vertices.extend_from_slice(vertices);
        self.indices.extend(indices.iter().map(|i| i + offset));
        self.min_z = self.min_z.min(z);

        true
    }

    pub fn flush(&mut self, queue: &Queue) -> Option<BufferSlice>
    {
        let vertices = std::mem::take(&mut self.vertices);
        let indices = std::mem::take(&mut self.indices);
        self.min_z = i32::MAX;

        self.pool.reset();

        if vertices.is_empty()
        {
            return None;
        }

        self.pool.write(queue, &vertices, &indices)
    }
}

use wgpu::{VertexAttribute, VertexBufferLayout, VertexStepMode, vertex_attr_array};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColoredVertex
{
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl ColoredVertex
{
    pub fn layout() -> VertexBufferLayout<'static>
    {
        const ATTRIBUTES: [VertexAttribute; 2] = vertex_attr_array![
            0 => Float32x2,
            1 => Float32x4,
        ];

        VertexBufferLayout {
            array_stride: size_of::<Self>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTRIBUTES,
        }
    }
}

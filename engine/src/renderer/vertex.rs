use wgpu::VertexBufferLayout;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex
{
    pub position: [f32; 2],
    pub texture_coords: [f32; 2],
}

impl Vertex
{
    pub fn layout() -> VertexBufferLayout<'static>
    {
        const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
            0 => Float32x2,  // position
            1 => Float32x2,  // tex_coords
        ];

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRIBS,
        }
    }
}

use crate::renderer::vertex::Vertex;

pub struct Mesh
{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Mesh
{
    pub fn quad(x: f32, y: f32, w: f32, h: f32) -> Self
    {
        Self {
            vertices: vec![
                Vertex {
                    position: [x, y],
                    texture_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [x + w, y],
                    texture_coords: [1.0, 0.0],
                },
                Vertex {
                    position: [x + w, y + h],
                    texture_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [x, y + h],
                    texture_coords: [0.0, 1.0],
                },
            ],
            indices: vec![0, 2, 1, 0, 3, 2],
        }
    }
}

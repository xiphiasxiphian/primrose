use glam::DVec2;

use crate::renderer::primitive::vertex::ColoredVertex;

type Color = [f32; 4];

#[derive(Clone, Copy, Debug)]
pub enum DrawCommand
{
    Line {
        start: DVec2,
        end: DVec2,
        thickness: f64,
        color: Color,
    },
    Circle {
        center: DVec2,
        radius: f64,
        color: Color,
        segments: u32,
    },
    FilledRect {
        pos: DVec2,
        size: DVec2,
        color: Color,
    }
}

impl DrawCommand
{
    pub fn tessellate(self, verts: &mut Vec<ColoredVertex>, inds: &mut Vec<u32>)
    {

    }
}

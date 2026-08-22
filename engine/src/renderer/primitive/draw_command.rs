use glam::DVec2;

use crate::renderer::primitive::vertex::ColoredVertex;

pub type Color = [f32; 4];

#[derive(Clone, Copy, Debug)]
pub enum DrawCommand
{
    Line
    {
        start: DVec2,
        end: DVec2,
        thickness: f64,
        color: Color,
    },
    Circle
    {
        center: DVec2,
        radius: f64,
        color: Color,
        segments: u32,
    },
    FilledRect
    {
        pos: DVec2, size: DVec2, color: Color
    },
}

impl DrawCommand
{
    pub fn tessellate(self, verts: &mut Vec<ColoredVertex>, indices: &mut Vec<u32>)
    {
        match self
        {
            Self::Line {
                start,
                end,
                thickness,
                color,
            } => Self::tessellate_line(verts, indices, start, end, thickness, color),
            Self::Circle {
                center,
                radius,
                color,
                segments,
            } => Self::tessellate_circle(verts, indices, center, radius, color, segments),
            Self::FilledRect { pos, size, color } => Self::tessellate_rect(verts, indices, pos, size, color),
        }
    }

    fn tessellate_line(
        verts: &mut Vec<ColoredVertex>,
        indices: &mut Vec<u32>,
        start: DVec2,
        end: DVec2,
        thickness: f64,
        color: Color,
    )
    {
        let Some(dir) = (end - start).try_normalize()
        else
        {
            return;
        };
        let nvec = dir.perp() * (thickness * 0.5);

        let base = verts.len() as u32;
        verts.extend_from_slice(&[
            ColoredVertex {
                position: (start + nvec).as_vec2().into(),
                color,
            },
            ColoredVertex {
                position: (start - nvec).as_vec2().into(),
                color,
            },
            ColoredVertex {
                position: (end - nvec).as_vec2().into(),
                color,
            },
            ColoredVertex {
                position: (end + nvec).as_vec2().into(),
                color,
            },
        ]);
        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    fn tessellate_circle(
        verts: &mut Vec<ColoredVertex>,
        indices: &mut Vec<u32>,
        center: DVec2,
        radius: f64,
        color: Color,
        segments: u32,
    )
    {
        let base = verts.len() as u32;

        verts.push(ColoredVertex {
            position: center.as_vec2().to_array(),
            color,
        });

        verts.extend((0..=segments).map(|i| {
            let angle = (i as f64 / segments as f64) * std::f64::consts::TAU;
            let pos = center + DVec2::from_angle(angle) * radius;

            ColoredVertex {
                position: pos.as_vec2().to_array(),
                color,
            }
        }));

        indices.extend((0..segments).flat_map(|i| [base, base + 1 + i, base + 2 + i]));
    }

    fn tessellate_rect(verts: &mut Vec<ColoredVertex>, indices: &mut Vec<u32>, pos: DVec2, size: DVec2, color: Color)
    {
        let base = verts.len() as u32;

        verts.extend_from_slice(
            &[pos, pos + size * DVec2::X, pos + size, pos + size * DVec2::Y].map(|x| ColoredVertex {
                position: x.as_vec2().to_array(),
                color,
            }),
        );

        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
}

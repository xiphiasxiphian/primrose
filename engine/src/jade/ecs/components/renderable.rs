use glam::DVec2;
use proc_macros::Component;

use crate::{
    jade::ecs::components::transform::Transform,
    renderer::{
        Renderable, ZIndex,
        mesh::Mesh,
        primitive::draw_command::{Color, DrawCommand},
    },
    util::assets::assetpool::TextureAsset,
};

#[derive(Component, Default)]
pub struct RenderInfo
{
    pub texture: Option<TextureAsset>,
    pub draw_commands: Vec<DrawCommand>,
    pub zindex: ZIndex,
}

impl RenderInfo
{
    pub fn line(&mut self, start: DVec2, end: DVec2, thickness: f64, color: Color)
    {
        self.draw_commands.push(DrawCommand::Line {
            start,
            end,
            thickness,
            color,
        });
    }

    pub fn circle(&mut self, center: DVec2, radius: f64, color: Color, segments: u32)
    {
        self.draw_commands.push(DrawCommand::Circle {
            center,
            radius,
            color,
            segments,
        });
    }

    pub fn filled_rect(&mut self, pos: DVec2, size: DVec2, color: Color)
    {
        self.draw_commands.push(DrawCommand::FilledRect { pos, size, color });
    }
}

impl<'a> Renderable for (&'a Transform, &'a RenderInfo)
{
    fn texture(&self) -> Option<&TextureAsset> { self.1.texture.as_ref() }

    fn mesh(&self) -> Mesh { self.0.mesh() }

    fn draw_commands(&self) -> &[DrawCommand] { &self.1.draw_commands }

    fn z_index(&self) -> ZIndex { self.1.zindex }
}

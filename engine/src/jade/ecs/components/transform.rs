use glam::DVec2;
use proc_macros::Component;

use crate::renderer::mesh::Mesh;

type Position = DVec2;
type Size = DVec2;

#[derive(Debug, Default, PartialEq, Copy, Clone, Component, derive_new::new)]
pub struct Transform
{
    pub pos: Position,
    pub size: Size,
}

impl Transform
{
    #[must_use]
    pub fn with_anchor(pos: Position, size: Size, anchor: Anchor) -> Self
    {
        let real_pos = anchor.to_top_left(pos, size);
        Transform { pos: real_pos, size }
    }

    #[must_use]
    pub fn scaled(&self, factor: f64) -> Self
    {
        let mut new = *self;
        new.scale(factor);

        new
    }

    pub fn scale(&mut self, factor: f64) { self.size = (self.size * factor).max(DVec2::ZERO) }

    #[must_use]
    pub fn stretched(&self, x_factor: f64, y_factor: f64) -> Self
    {
        let mut new = *self;
        new.stretch(x_factor, y_factor);

        new
    }

    pub fn stretch(&mut self, x_factor: f64, y_factor: f64)
    {
        self.size *= DVec2::new(x_factor, y_factor).max(DVec2::ZERO);
    }

    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "truncation will only happen at values where a problem will happen elsewhere first")]
    pub fn mesh(&self) -> Mesh
    {
        Mesh::quad(
            self.pos.x as f32,
            self.pos.y as f32,
            self.size.x as f32,
            self.size.y as f32,
        )
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Anchor
{
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Anchor
{
    #[must_use]
    pub fn to_top_left(self, Position { x, y }: Position, Size { x: w, y: h }: Size) -> Position
    {
        match self
        {
            Anchor::TopLeft => (x, y),
            Anchor::TopRight => (x - w, y),
            Anchor::BottomLeft => (x, y - h),
            Anchor::BottomRight => (x - w, y - h),
            Anchor::Center => (x - (w / 2.0), y - (h / 2.0)),
        }
        .into()
    }

    #[must_use]
    pub fn to_anchor(self, target: Self, old_pos: Position, size @ Size { x: width, y: height }: Size) -> Position
    {
        let DVec2 { x, y } = self.to_top_left(old_pos, size);
        match target
        {
            Anchor::TopLeft => (x, y),
            Anchor::TopRight => (x + width, y),
            Anchor::BottomLeft => (x, y + height),
            Anchor::BottomRight => (x + width, y + height),
            Anchor::Center => (x + (width / 2.0), y + (height / 2.0)),
        }
        .into()
    }
}

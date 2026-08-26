use glam::DVec2;
use proc_macros::Component;

use crate::jade::ecs::components::Component;


type Position = DVec2;
type Size = DVec2;

#[derive(Debug, Default, PartialEq, Copy, Clone, Component)]
pub struct Transform
{
    pub pos: Position,
    pub size: Size,
}

impl Transform
{
    pub fn with_anchor(pos: Position, size: Size, anchor: Anchor) -> Self
    {
        let real_pos = anchor.to_top_left(pos, size);
        Transform { pos: real_pos, size }
    }

    pub fn scaled(&self, factor: f64) -> Self
    {
        let mut new = *self;
        new.scale(factor);

        new
    }

    pub fn scale(&mut self, factor: f64) { self.size = (self.size * factor).max(DVec2::ZERO) }

    pub fn stretched(&self, x_factor: f64, y_factor: f64) -> Self
    {
        let mut new = *self;
        new.stretch(x_factor, y_factor);

        new
    }

    pub fn stretch(&mut self, x_factor: f64, y_factor: f64)
    {
        self.size = self.size * DVec2::new(x_factor, y_factor).max(DVec2::ZERO);
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

    pub fn to_anchor(self, target: Self, old_pos: Position, size @ Size { x: w, y: h }: Size) -> Position
    {
        let DVec2 { x, y } = self.to_top_left(old_pos, size);
        match target
        {
            Anchor::TopLeft => (x, y),
            Anchor::TopRight => (x + w, y),
            Anchor::BottomLeft => (x, y + h),
            Anchor::BottomRight => (x + w, y + h),
            Anchor::Center => (x + (w / 2.0), y + (h / 2.0)),
        }
        .into()
    }
}

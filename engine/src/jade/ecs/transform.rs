type Position = (f64, f64);
type Size = (f64, f64);

#[derive(Debug, Default, PartialEq, PartialOrd, Copy, Clone)]
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

    pub fn scale(&mut self, factor: f64)
    {
        self.size = ((self.size.0 * factor).max(0.0), (self.size.1 * factor).max(0.0))
    }

    pub fn stretched(&self, x_factor: f64, y_factor: f64) -> Self
    {
        let mut new = *self;
        new.stretch(x_factor, y_factor);

        new
    }

    pub fn stretch(&mut self, x_factor: f64, y_factor: f64)
    {
        self.size = ((self.size.0 * x_factor).max(0.0), (self.size.1 * y_factor).max(0.0))
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
    pub fn to_top_left(self, pos @ (x, y): Position, (w, h): Size) -> Position
    {
        match self
        {
            Anchor::TopLeft => pos,
            Anchor::TopRight => (x - w, y),
            Anchor::BottomLeft => (x, y - h),
            Anchor::BottomRight => (x - w, y - h),
            Anchor::Center => (x - (w / 2.0), y - (h / 2.0)),
        }
    }

    pub fn to_anchor(self, target: Self, old_pos: Position, size @ (w, h): Size) -> Position
    {
        let pos @ (x, y) = self.to_top_left(old_pos, size);
        match target
        {
            Anchor::TopLeft => pos,
            Anchor::TopRight => (x + w, y),
            Anchor::BottomLeft => (x, y + h),
            Anchor::BottomRight => (x + w, y + h),
            Anchor::Center => (x + (w / 2.0), y + (h / 2.0)),
        }
    }
}

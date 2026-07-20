use itertools::Itertools;

pub struct Color
{
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color
{
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }

    pub fn from_hex(code: &str, a: u8) -> Option<Self>
    {
        let r = u8::from_str_radix(code.get(0..2)?, 16).ok()?;
        let g = u8::from_str_radix(code.get(2..4)?, 16).ok()?;
        let b = u8::from_str_radix(code.get(4..6)?, 16).ok()?;

        Some(Self { r, g, b, a })
    }

    pub fn to_floats(&self) -> (f64, f64, f64, f64)
    {
        [self.r, self.g, self.b, self.a]
            .iter()
            .map(|x| (*x as f64 / u8::MAX as f64).clamp(0.0, 1.0))
            .collect_tuple()
            .expect("")
    }
}

impl From<Color> for wgpu::Color
{
    fn from(value: Color) -> Self
    {
        let (r, g, b, a) = value.to_floats();
        Self { r, g, b, a }
    }
}

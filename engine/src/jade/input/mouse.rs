use strum::EnumCount;

#[derive(Debug, EnumCount)]
pub enum MouseButton
{
    Left,
    Middle,
    Right,
}

impl TryFrom<i16> for MouseButton
{
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error>
    {
        match value
        {
            0 => Ok(Self::Left),
            1 => Ok(Self::Middle),
            2 => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

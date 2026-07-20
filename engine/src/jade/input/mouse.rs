use strum::EnumCount;
use winit::event::MouseButton as NativeMouseButton;

#[derive(Debug, EnumCount)]
pub enum MouseButton
{
    Left,
    Middle,
    Right,
}

impl TryFrom<NativeMouseButton> for MouseButton
{
    type Error = ();

    fn try_from(value: NativeMouseButton) -> Result<Self, Self::Error>
    {
        match value
        {
            NativeMouseButton::Left => Ok(Self::Left),
            NativeMouseButton::Middle => Ok(Self::Middle),
            NativeMouseButton::Right => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

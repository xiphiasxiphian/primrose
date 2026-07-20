use glam::{Mat4, Vec2, Vec3, camera::rh::proj::directx::orthographic};

pub struct Camera
{
    pub position: Vec2,
    zoom: f32,
    viewport: Vec2,
}

impl Camera
{
    pub fn new(viewport_dims: (f32, f32)) -> Self
    {
        Self {
            position: Vec2::ZERO,
            zoom: 1.0,
            viewport: Vec2::new(viewport_dims.0, viewport_dims.1),
        }
    }

    pub fn update_viewport(&mut self, (w, h): (f32, f32)) { self.viewport = Vec2::new(w, h) }

    pub fn set_zoom(&mut self, zoom: f32) -> f32
    {
        self.zoom = zoom.max(0.0);
        self.zoom
    }

    pub fn adjust_zoom(&mut self, multiplier: f32) -> f32
    {
        self.zoom = (self.zoom * multiplier).max(0.0);
        self.zoom
    }

    pub fn view_projection(&self) -> Mat4
    {
        let prog = orthographic(0.0, self.viewport.x, self.viewport.y, 0.0, -1.0, 1.0);

        let half = self.viewport / 2.0;
        let view = Mat4::from_scale(Vec3::new(self.zoom, self.zoom, 1.0))
            * Mat4::from_translation(Vec3::new(
                -self.position.x + half.x / self.zoom,
                -self.position.y + half.y / self.zoom,
                0.0,
            ));

        prog * view
    }
}

use glam::Vec2;

pub fn pixels_to_clip(x: f32, y: f32, width: f32, height: f32) -> [f32; 2] {
    [(2. * x / width) - 1., 1. - (2. * y / height)]
    // [(x / width - 0.5) * 2., (1. - y / height - 0.5) * 2.]
}

pub fn pixels_to_texture_coord(x: f32, y: f32, width: f32, height: f32) -> [f32; 2] {
    [x / width, y / height]
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn position(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    /// Check if a point is contained by `Rect`
    ///
    ///  # Arguments
    ///
    /// * `point` - The point to test
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width
            && point.y >= self.y
            && point.y < self.y + self.height
    }
}

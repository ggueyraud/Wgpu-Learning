use derive_more::From;

/// Describe color as RGB format
#[derive(From, Clone, Copy, Debug)]
pub struct Color(u8, u8, u8);

#[allow(dead_code)]
pub const BLACK: Color = Color(0, 0, 0);
pub const WHITE: Color = Color(255, 255, 255);
pub const RED: Color = Color(255, 0, 0);
pub const GREEN: Color = Color(0, 255, 0);
pub const BLUE: Color = Color(0, 0, 255);

impl Into<[f32; 3]> for Color {
    fn into(self) -> [f32; 3] {
        [
            (self.0 / 255) as f32,
            (self.1 / 255) as f32,
            (self.2 / 255) as f32,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn into_accepted_wgsl_color_format() {
        let color: [f32; 3] = Color(255, 255, 255).into();
        assert_eq!(color, [1., 1., 1.]);
        let color: [f32; 3] = Color(155, 155, 155).into();
        assert_eq!(color, [(155 / 255) as f32; 3])
    }
}

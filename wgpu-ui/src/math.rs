pub fn pixels_to_clip(x: f32, y: f32, width: f32, height: f32) -> [f32; 2] {
    [(2. * x / width) - 1., 1. - (2. * y / height)]
}

pub fn pixels_to_texture_coord(x: f32, y: f32, width: f32, height: f32) -> [f32; 2] {
    [x / width, y / height]
}

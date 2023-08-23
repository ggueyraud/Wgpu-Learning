use crate::math::Rect;

pub struct Animation {
    frames: Vec<Rect>,
    current_frame: usize,
    frame_time: f32,
    elapsed: f32,
}

impl Animation {
    pub fn new(frames: &[Rect], frame_time: f32) -> Self {
        Self {
            frames: frames.to_owned(),
            current_frame: 0,
            frame_time,
            elapsed: 0.,
        }
    }

    pub fn get_frame(&self) -> Option<&Rect> {
        self.frames.get(self.current_frame)
    }

    pub fn reset(&mut self) {
        self.current_frame = 0;
        self.elapsed = 0.;
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.elapsed += dt;

        if self.elapsed > self.frame_time {
            self.elapsed -= self.frame_time;

            if self.current_frame + 1 < self.frames.len() {
                self.current_frame += 1;
            } else {
                self.current_frame = 0;
            }

            return true;
        }

        false
    }
}

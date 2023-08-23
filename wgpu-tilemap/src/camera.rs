use winit::event::{WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseScrollDelta, TouchPhase};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub position: cgmath::Vector2<f32>,
    pub viewport_size: cgmath::Vector2<f32>,
    pub scale: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let translation = cgmath::Matrix4::from_translation(cgmath::Vector2::new(-self.position.x, -self.position.y).extend(0.));
        let scale = cgmath::Matrix4::from_nonuniform_scale(self.scale, self.scale, 1.0);

        scale * translation
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4]
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;

        Self {
            view_proj: cgmath::Matrix4::identity().into()
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct Controller {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_zooming_in: bool,
    is_zooming_out: bool
}

impl Controller {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_backward_pressed: false,
            is_forward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_zooming_in: false,
            is_zooming_out: false
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input: KeyboardInput {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
                },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::Z => {
                        self.is_zooming_in = is_pressed;
                        true
                    }
                    VirtualKeyCode::S => {
                        self.is_zooming_out = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        if self.is_left_pressed {
            camera.position.x -= self.speed;
        }

        if self.is_right_pressed {
            camera.position.x += self.speed;
        }

        if self.is_zooming_in {
            camera.scale += self.speed;
        }
        if self.is_zooming_out {
            camera.scale -= self.speed;
        }

        if self.is_forward_pressed {
            camera.position.y += self.speed;
        }
        if self.is_backward_pressed {
            camera.position.y -= self.speed;
        }
    }
}
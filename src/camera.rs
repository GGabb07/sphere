use std::f32::consts::FRAC_PI_2;

use glfw::*;
use nalgebra_glm::*;

pub struct Camera {
    pos: Vec3,
    front: Vec3,
    right: Vec3,
    pitch: f32,
    yaw: f32,
    speed: f32,
    sensitivity: f32,
    perspective: Mat4,
}

impl Camera {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        let perspective =
            Mat4::new_perspective(1280_f32 / 720_f32, 70_f32.to_radians(), 0.0001, 1000.);
        Self {
            pos: vec3(-8., -8., -8.),
            front: vec3(0., 0., 1.),
            right: vec3(1., 0., 0.),
            pitch: 0.,
            yaw: FRAC_PI_2,
            speed,
            sensitivity,
            perspective,
        }
    }

    pub fn update(&mut self, dt: f32, win: &Window, output: &mut Mat4) {
        let mut delta = dt;
        if let Action::Press = win.get_key(Key::T) {
            self.pos.fill(-8.);
            self.pitch = 0.;
            self.yaw = FRAC_PI_2;
            delta = 0.;
        }

        if let Action::Release = win.get_key(Key::Right) {
        } else {
            self.yaw += self.sensitivity;
        }
        if let Action::Release = win.get_key(Key::Left) {
        } else {
            self.yaw -= self.sensitivity;
        }
        if let Action::Release = win.get_key(Key::Down) {
        } else {
            self.pitch += self.sensitivity;
        }
        if let Action::Release = win.get_key(Key::Up) {
        } else {
            self.pitch -= self.sensitivity;
        }

        if self.pitch > FRAC_PI_2 {
            self.pitch = FRAC_PI_2;
        }
        if self.pitch < -FRAC_PI_2 {
            self.pitch = -FRAC_PI_2;
        }

        let cos_pitch = self.pitch.cos();
        let right_yaw = self.yaw - FRAC_PI_2;
        self.front.x = -self.yaw.sin() * cos_pitch;
        self.front.y = self.pitch.sin();
        self.front.z = self.yaw.cos() * cos_pitch;
        self.right.x = -right_yaw.sin() * cos_pitch;
        self.right.y = 0.;
        self.right.z = right_yaw.cos() * cos_pitch;

        let speed = if let Action::Release = win.get_key(Key::Tab) {
            self.speed
        } else {
            self.speed * 2.
        } * delta;

        if let Action::Release = win.get_key(Key::W) {
        } else {
            self.pos += self.front * speed;
        }
        if let Action::Release = win.get_key(Key::S) {
        } else {
            self.pos -= self.front * speed;
        }
        if let Action::Release = win.get_key(Key::A) {
        } else {
            self.pos += self.right * speed;
        }
        if let Action::Release = win.get_key(Key::D) {
        } else {
            self.pos -= self.right * speed;
        }
        if let Action::Release = win.get_key(Key::Space) {
        } else {
            self.pos.y -= speed;
        }
        if let Action::Release = win.get_key(Key::LeftShift) {
        } else {
            self.pos.y += speed;
        }

        if let Action::Release = win.get_key(Key::I) {
        } else {
            self.pos.z += speed;
        }
        if let Action::Release = win.get_key(Key::K) {
        } else {
            self.pos.z -= speed;
        }
        if let Action::Release = win.get_key(Key::L) {
        } else {
            self.pos.x += speed;
        }
        if let Action::Release = win.get_key(Key::J) {
        } else {
            self.pos.x -= speed;
        }

        // output.fill_with_identity();
        *output = self.perspective;
        *output = rotate_x(output, self.pitch);
        *output = rotate_y(output, self.yaw);
        *output = translate(output, &self.pos);
    }

    pub fn rotate(&mut self, dt_x: f32, dt_y: f32) {
        self.pitch += dt_y * self.sensitivity;
        self.yaw += dt_x * self.sensitivity;
    }
}

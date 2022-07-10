use glam::Vec3;

use crate::{
    camera::Camera,
    input::{keyboard, Key, Keyboard},
    resources::Resources,
};

pub struct PositionIntegrator {
    origin: Vec3,
    accumulator: Vec3,
    start_time_secs: f32,
    last_update_secs: f32,
}

impl PositionIntegrator {
    pub fn new(origin: Vec3, time: f32) -> Self {
        Self {
            origin,
            accumulator: Vec3::ZERO,
            start_time_secs: time,
            last_update_secs: time,
        }
    }

    pub fn update(&mut self, keyboard: Option<&mut Keyboard>, camera: &Camera, speed: f32, time_secs: f32) -> Vec3 {
        if let Some(keyboard) = keyboard {
            let right = keyboard.get_axis(Key::D, Key::A);
            let up = keyboard.get_axis(Key::Space, Key::LShift);
            let fwd = keyboard.get_axis(Key::W, Key::S);

            if right != 0 || up != 0 || fwd != 0 {
                let (ys, yc) = camera.yaw().sin_cos();
                let fwd_dir = Vec3::new(yc, 0.0, ys);
                let up_dir = Vec3::Y;
                let right_dir = fwd_dir.cross(up_dir);

                let velocity =
                    (right as f32) * right_dir + (fwd as f32) * fwd_dir + (up as f32) * up_dir;

                let speed = speed * (time_secs - self.last_update_secs);
                self.accumulator += velocity.normalize() * speed;
            }
        }
        self.last_update_secs = time_secs;
        self.origin + self.accumulator
    }

    pub fn end_network_tick(&mut self) -> Vec3 {
        let accum = self.accumulator;
        self.accumulator = Vec3::ZERO;

        let pos = self.origin + accum;
        self.origin = pos;
        
        self.start_time_secs = self.last_update_secs;
        
        accum
    }
}

pub struct InputRecorder {
    pub integrator: PositionIntegrator
}

impl InputRecorder {
    pub fn new(position: Vec3, time_secs: f32) -> Self {
        Self {
            integrator: PositionIntegrator::new(position, time_secs)
        }
    }
}

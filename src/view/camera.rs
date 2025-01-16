use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};

#[derive(Component)]
pub struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            pitch: -std::f32::consts::FRAC_PI_4,
            yaw: std::f32::consts::FRAC_PI_4,
        }
    }
}

pub fn spawn_camera() -> (Camera3d, PanOrbitCamera) {
    (
        Camera3d::default(),
        PanOrbitCamera {
            focus: Vec3::new(0.0, 1.0, 0.0),
            radius: 400.0,
            pitch: -15.0f32.to_radians(),
            yaw: 0.0f32.to_radians(),
        }
    )
}

pub fn pan_orbit_camera(
    mut query: Query<(&mut PanOrbitCamera, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
) {
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Left;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;

    // Handle mouse motion for orbit and pan
    for ev in mouse_motion.read() {
        if mouse_button.pressed(orbit_button) {
            rotation_move += ev.delta;
        } else if mouse_button.pressed(pan_button) && keyboard.pressed(KeyCode::AltLeft) {
            pan += ev.delta;
        }
    }
    
    // Handle scroll for zoom
    for ev in mouse_wheel.read() {
        scroll += ev.y;
    }

    for (mut pan_orbit, mut transform) in query.iter_mut() {
        let mut any_change = false;

        // Handle orbit (rotation)
        if rotation_move.length_squared() > 0.0 {
            any_change = true;
            let sensitivity = 0.004;
            pan_orbit.yaw -= rotation_move.x * sensitivity;
            pan_orbit.pitch -= rotation_move.y * sensitivity;
            
            // Clamp pitch to prevent camera flipping
            pan_orbit.pitch = pan_orbit.pitch.clamp(
                -std::f32::consts::FRAC_PI_2 * 0.99,
                std::f32::consts::FRAC_PI_2 * 0.99,
            );
        }

        // Handle pan
        if pan.length_squared() > 0.0 {
            any_change = true;
            let pan_sensitivity = 0.002;
            let right = transform.right() * -pan.x * pan_orbit.radius * pan_sensitivity;
            let up = transform.up() * pan.y * pan_orbit.radius * pan_sensitivity;
            pan_orbit.focus += right;
            pan_orbit.focus += up;
        }

        // Handle zoom
        if scroll.abs() > 0.0 {
            any_change = true;
            let scroll_sensitivity = if keyboard.pressed(KeyCode::ShiftLeft) { 0.5 } else { 0.1 };
            pan_orbit.radius *= (1.0 - scroll.signum() * scroll_sensitivity).max(0.0001);
        }

        // Update transform if anything changed
        if any_change {
            let rot_matrix = Mat3::from_euler(EulerRot::YXZ, pan_orbit.yaw, pan_orbit.pitch, 0.0);
            transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
            transform.look_at(pan_orbit.focus, Vec3::Y);
        }
    }
}

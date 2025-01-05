use bevy::{color::palettes::tailwind::{self, *}, prelude::Color};

pub const YELLOW: Color = Color::linear_rgb(255.0, 255.0, 0.0);
pub const BLACK: Color = Color::linear_rgb(0.0, 0.0, 0.0);
pub const GRAY: Color = Color::linear_rgb(20.0, 20.0, 20.0);
pub const WHITE: Color = Color::linear_rgb(255.0, 255.0, 255.0);


pub const RED: Color = Color::linear_rgb(255.0, 0.0, 0.0);
pub const GREEN: Color = Color::linear_rgb(0.0, 255.0, 0.0);
pub const BLUE: Color = Color::linear_rgb(0.0, 0.0, 255.0);

//button colors
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);


//part colors
pub const NO_CHANGE_COLOR: Color = Color::WHITE;
pub const UNUSED_COLOR: Color = Color::srgb(0.87, 0.87, 0.87); // Assuming GRAY_300 is approximately this value
pub const HOVER_COLOR: Color = Color::srgb(CYAN_300.red, CYAN_300.green, CYAN_300.blue);
pub const PRESSED_COLOR: Color = Color::srgb(YELLOW_300.red, YELLOW_300.green, YELLOW_300.blue);
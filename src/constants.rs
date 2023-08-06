use sdl2::pixels::Color;

pub const WINDOW_WIDTH: u32 = 640;
pub const WINDOW_HEIGHT: u32 = 320;

pub const DISPLAY_WIDTH: u8 = 64;
pub const DISPLAY_HEIGHT: u8 = 32;

pub const PIXEL_WIDTH: u8 = WINDOW_WIDTH as u8 / DISPLAY_WIDTH;
pub const PIXEL_HEIGHT: u8 = WINDOW_HEIGHT as u8 / DISPLAY_HEIGHT;

pub const PIXEL_COLOR: &Color = &Color::RGB(255, 255, 255);
pub const BACKGROUND_COLOR: &Color = &Color::RGB(0, 0, 0);

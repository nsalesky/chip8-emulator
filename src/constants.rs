pub const CANVAS_WIDTH: usize = 64;
pub const CANVAS_HEIGHT: usize = 32;
pub const PIXEL_COLOR: &[f32; 3] = &[1.0, 1.0, 1.0];
pub const BACKGROUND_COLOR: &[f32; 3] = &[0.0, 0.0, 0.0];

// A projection matrix that transforms coordinates from the range ([-0.5, 0.5], [-0.5, 0.5]) to
// ([0.0, CANVAS_WIDTH], [0.0, CANVAS_HEIGHT])
pub const PROJECTION_MATRIX: &[[f32; 4]; 4] = &[
    [2.0 / CANVAS_WIDTH as f32, 0.0, 0.0, 0.0],
    [0.0, 2.0 / CANVAS_HEIGHT as f32, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [-1.0, -1.0, 0.0, 1.0],
];

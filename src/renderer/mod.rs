mod gl_canvas;
mod vertex;

use anyhow::Result;
pub use gl_canvas::GLCanvas;
use glium::Frame;

pub trait CanvasRenderer {
    fn draw(&self, frame: &mut Frame) -> Result<()>;
    fn set_pixel(&mut self, row: usize, col: usize, state: bool) -> Result<()>;
    fn get_pixel(&self, row: usize, col: usize) -> Result<bool>;
}

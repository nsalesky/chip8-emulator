use crate::constants::{CANVAS_HEIGHT, CANVAS_WIDTH, PIXEL_COLOR, PROJECTION_MATRIX};
use anyhow::Result;
use glium::{
    index::PrimitiveType, Display, DrawParameters, Frame, IndexBuffer, Program, Rect, Surface,
    VertexBuffer,
};

use super::CanvasRenderer;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

pub struct GLCanvas {
    pixels: [bool; CANVAS_WIDTH * CANVAS_HEIGHT],
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    program: Program,
}

impl GLCanvas {
    pub fn new(display: &Display) -> Result<Self> {
        let vertices = [
            Vertex {
                position: [0.5, 0.5],
                color: *PIXEL_COLOR,
            }, // top right
            Vertex {
                position: [0.5, -0.5],
                color: *PIXEL_COLOR,
            }, // bottom right
            Vertex {
                position: [-0.5, -0.5],
                color: *PIXEL_COLOR,
            }, // bottom left
            Vertex {
                position: [-0.5, 0.5],
                color: *PIXEL_COLOR,
            }, // top left
        ];

        let indices = [
            0, 1, 3, // first triangle
            1, 2, 3, // second triangle
        ];

        let vertex_buffer = VertexBuffer::new(display, &vertices)?;
        let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, &indices)?;

        let program = program!(display,
            330 => {
                vertex: include_str!("../../shaders/canvas.vert"),
                fragment: include_str!("../../shaders/canvas.frag"),
            },
        )?;

        Ok(GLCanvas {
            pixels: [false; CANVAS_WIDTH * CANVAS_HEIGHT],
            vertex_buffer,
            index_buffer,
            program,
        })
    }

    fn draw_pixel(&self, frame: &mut Frame, row: usize, col: usize) -> Result<()> {
        let uniforms = uniform! {
            projectionMatrix: *PROJECTION_MATRIX,
            modelMatrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [col as f32, row as f32, 0.0, 1.0],
            ],
        };

        frame.draw(
            &self.vertex_buffer,
            &self.index_buffer,
            &self.program,
            &uniforms,
            &Default::default(),
        )?;
        Ok(())
    }
}

impl CanvasRenderer for GLCanvas {
    fn set_pixel(&mut self, row: usize, col: usize, state: bool) -> Result<()> {
        check_bounds(row, col)?;
        self.pixels[(row * CANVAS_WIDTH) + col] = state;
        Ok(())
    }

    fn get_pixel(&self, row: usize, col: usize) -> Result<bool> {
        check_bounds(row, col)?;
        Ok(self.pixels[(row * CANVAS_WIDTH) + col])
    }

    fn draw(&self, frame: &mut Frame) -> Result<()> {
        for row in 0..CANVAS_HEIGHT {
            for col in 0..CANVAS_WIDTH {
                if self.get_pixel(row, col)? {
                    self.draw_pixel(frame, row, col)?;
                }
            }
        }

        Ok(())
    }
}

fn check_bounds(row: usize, col: usize) -> Result<()> {
    if row >= CANVAS_HEIGHT || col >= CANVAS_WIDTH {
        bail!("Position ({}, {}) is out of bounds", row, col,);
    } else {
        Ok(())
    }
}

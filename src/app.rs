use crate::grid::Grid;
use crate::particle::Particle;

pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::Clamped;
pub use web_sys::{CanvasRenderingContext2d, ImageData};

type Rect = (i32, i32, i32, i32, (u8, u8, u8));

#[derive(Clone, Copy, Default, Debug)]
struct InputState {
    mouse_clicked: [bool; 3],
    mouse_holding: [bool; 3],
    mouse_position: (i32, i32),
}

#[derive(Clone, Copy, Default, Debug)]
struct CanvasState {
    width: u32,
    height: u32,
    pixel_size: i32,
    brush_size: i32,
    ups: usize,
}

#[wasm_bindgen]
pub struct AppState {
    cells: Grid<Particle>,
    brush: Option<Particle>,
    pixel_data: Vec<u8>,
    input_state: InputState,
    canvas_state: CanvasState,
}

#[wasm_bindgen]
impl AppState {
    pub fn new(w: usize, h: usize) -> Self {
        let canvas_state = CanvasState {
            width: w as u32,
            height: h as u32,
            pixel_size: 5,
            brush_size: 5,
            ups: 5,
        };
        Self {
            cells: Grid::new(
                w / canvas_state.pixel_size as usize,
                h / canvas_state.pixel_size as usize,
            ),
            brush: None,
            pixel_data: vec![0x00; w * h * 4],
            input_state: InputState::default(),
            canvas_state,
        }
    }

    pub fn on_mousedown(&mut self, button: i32) {
        if button < 3 {
            self.input_state.mouse_holding[button as usize] = true;
            self.input_state.mouse_clicked[button as usize] = true;
        }
    }

    pub fn on_mouseup(&mut self, button: i32) {
        if button < 3 {
            self.input_state.mouse_holding[button as usize] = false;
        }
    }

    pub fn on_mousemove(&mut self, x: i32, y: i32) {
        self.input_state.mouse_position = (x, y);
    }

    pub fn tick(&mut self, canvas_ctx: &CanvasRenderingContext2d) {
        canvas_ctx.set_image_smoothing_enabled(false);
        self.handle_events();
        self.update();
        self.draw(canvas_ctx);
    }

    fn reset_input_state(&mut self) {
        self.input_state.mouse_clicked = [false; 3]
    }

    fn handle_events(&mut self) {
        if self.input_state.mouse_clicked[2] {
            self.brush = match self.brush {
                Some(Particle::Sand) => Some(Particle::Water),
                Some(Particle::Water) => Some(Particle::Wall),
                Some(Particle::Wall) => Some(Particle::Sand),
                _ => Some(Particle::Sand),
            };
        } else if self.input_state.mouse_clicked[1] {
            self.brush = None;
        }
        web_sys::console::log_1(&format!("brush: {:?}", self.brush).into());

        if self.input_state.mouse_holding[0] {
            let (x, y) = self.input_state.mouse_position;
            let (x, y) = (
                x / self.canvas_state.pixel_size as i32,
                y / self.canvas_state.pixel_size as i32,
            );

            for dx in -self.canvas_state.brush_size / 2..self.canvas_state.brush_size / 2 {
                for dy in -self.canvas_state.brush_size / 2..self.canvas_state.brush_size / 2 {
                    if let Some(index) = self.cells.get_index((x + dx) as usize, (y + dy) as usize)
                    {
                        self.cells.data[index] = match self.brush {
                            Some(particle) => particle,
                            None => Particle::Empty,
                        };
                    }
                }
            }
        }

        self.reset_input_state();
    }

    fn update(&mut self) {
        for _ in 0..self.canvas_state.ups {
            let mut grid_copy = self.cells.clone();
            self.cells
                .iter()
                .enumerate()
                .rev()
                .for_each(|(i, particle)| {
                    let (x, y) = self.cells.get_coords(i).unwrap();
                    let (x, y) = (x as i32, y as i32);

                    particle.update(x, y, &mut grid_copy)
                });
            self.cells = grid_copy;
        }
    }

    fn draw_rect(&mut self, rect: Rect) {
        let (x, y, w, h, (r, g, b)) = rect;
        for ny in y..y + h {
            if ny as u32 >= self.canvas_state.height {
                continue;
            }

            for nx in x..x + w {
                if nx as u32 >= self.canvas_state.width {
                    continue;
                }

                let index = ((ny * self.canvas_state.width as i32 + nx) * 4) as usize;
                self.pixel_data[index..index + 4].copy_from_slice(&[r, g, b, 0xff]);
            }
        }
    }

    fn draw(&mut self, ctx: &CanvasRenderingContext2d) {
        self.pixel_data
            .chunks_exact_mut(4)
            .for_each(|chunk| chunk.copy_from_slice(&[0x15, 0x15, 0x15, 0xff]));
        let (mx, my) = self.input_state.mouse_position;

        let rects: Vec<Option<Rect>> = self
            .cells
            .iter()
            .enumerate()
            .map(|(i, particle)| {
                particle.color().map(|color| {
                    let (x, y) = self.cells.get_coords(i).unwrap();
                    let (x, y) = (x as i32, y as i32);

                    (
                        x * self.canvas_state.pixel_size as i32,
                        y * self.canvas_state.pixel_size as i32,
                        self.canvas_state.pixel_size,
                        self.canvas_state.pixel_size,
                        color,
                    )
                })
            })
            .collect();

        rects.iter().for_each(|rect| {
            rect.map(|rect| self.draw_rect(rect));
        });
        self.draw_rect((
            mx - (self.canvas_state.brush_size * self.canvas_state.pixel_size) / 2,
            my - (self.canvas_state.brush_size * self.canvas_state.pixel_size) / 2,
            self.canvas_state.brush_size * self.canvas_state.pixel_size,
            self.canvas_state.brush_size * self.canvas_state.pixel_size,
            (0xc8, 0x6d, 0x56),
        ));

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.pixel_data),
            self.canvas_state.width,
            self.canvas_state.height,
        )
        .expect("failed to create image data from pixel data");

        ctx.put_image_data(&data, 0.0, 0.0)
            .expect("failed to render image data");

        ctx.set_font("15px monospace");
        ctx.set_fill_style(&"#d0d0d0".into());
        ctx.fill_text("  left click: place something", 10.0, 20.0)
            .expect("failed to paint text");
        ctx.fill_text("middle click: pick the eraser", 10.0, 40.0)
            .expect("failed to paint text");
        ctx.fill_text(" right click: switch the brush", 10.0, 60.0)
            .expect("failed to paint text");
        ctx.fill_text(
            format!("       brush: {:?}", self.brush).as_str(),
            10.0,
            80.0,
        )
        .expect("failed to paint text");
    }
}

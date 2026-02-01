use std::cmp::{max, min};

use crate::DisplayController;

impl DisplayController {
    pub fn line(&mut self, x1: isize, y1: isize, x2: isize, y2: isize, color: usize) {
        if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                self.set_pixel(x, y1, color);
            }
            return;
        }

        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                self.set_pixel(x1, y, color);
            }
            return;
        }

        let dx: isize = (x2 - x1).abs();
        let dy: isize = -(y2 - y1).abs();
        let sx: isize = if x1 < x2 { 1 } else { -1 };
        let sy: isize = if y1 < y2 { 1 } else { -1 };
        let mut error = dx + dy;

        let mut x0 = x1;
        let mut y0 = y1;
        let x1 = x2;
        let y1 = y2;

        loop {
            self.set_pixel(x0, y0, color);

            if x0 == x1 && y0 == y1 {
                break;
            };
            let e2 = 2 * error;

            if e2 >= dy {
                if x0 == x1 {
                    break;
                };
                error += dy;
                x0 += sx;
            }

            if e2 <= dx {
                if y0 == y1 {
                    break;
                };
                error += dx;
                y0 += sy;
            }
        }
    }

    pub fn vector(&mut self, x: isize, y: isize, l: isize, color: usize, a: f32) -> (isize, isize) {
        let x1 = x;
        let y1 = y;

        let x_move = a.cos() * l as f32;
        let y_move = a.sin() * l as f32;

        let x2 = if x_move < 0.0 {
            x1 - (-x_move).round() as isize
        } else {
            x1 + x_move.round() as isize
        };

        let y2 = if y_move < 0.0 {
            y1 - (-y_move).round() as isize
        } else {
            y1 + y_move.round() as isize
        };

        self.line(x1, y1, x2, y2, color);

        (x2, y2)
    }

    pub fn square(
        &mut self,
        x: isize,
        y: isize,
        width: isize,
        height: isize,
        color: usize,
        fill_color: Option<usize>,
    ) {
        self.line(x, y, x + width - 1, y, color);
        self.line(x + width - 1, y, x + width - 1, y + height - 1, color);
        self.line(x + width - 1, y + height - 1, x, y + height - 1, color);
        self.line(x, y + height - 1, x, y, color);

        if let Some(fill) = fill_color {
            for y in (y + 1)..(y + height - 1) {
                self.line(x + 1, y, x + width - 2, y, fill);
            }
        }
    }
}
use crate::DisplayController;

impl DisplayController {

    fn draw_circle(
        &mut self,
        xc: isize,
        yc: isize,
        x: isize,
        y: isize,
        color: usize,
        fill_color: Option<usize>,
    ) {
        self.set_pixel(xc + x, yc + y, color);
        self.set_pixel(xc - x, yc + y, color);
        self.set_pixel(xc + x, yc - y, color);
        self.set_pixel(xc - x, yc - y, color);
        self.set_pixel(xc + y, yc + x, color);
        self.set_pixel(xc - y, yc + x, color);
        self.set_pixel(xc + y, yc - x, color);
        self.set_pixel(xc - y, yc - x, color);

        if let Some(fill) = fill_color {
            self.line(xc - x, yc + y - 1, xc + x, yc + y - 1, fill);
            self.line(xc - x, yc - y + 1, xc + x, yc - y + 1, fill);
            self.line(xc - y + 1, yc + x, xc + y - 1, yc + x, fill);
            self.line(xc - y + 1, yc - x, xc + y - 1, yc - x, fill);
        }
    }

    pub fn circle(
        &mut self,
        xc: isize,
        yc: isize,
        r: usize,
        color: usize,
        fill_color: Option<usize>,
    ) {
        let mut x: isize = 0;
        let mut y: isize = r as isize;
        let mut d: isize = 3 - 2 * r as isize;

        //Special case for r = 1
        if r == 1 {
            self.set_pixel(xc, yc + 1, color);
            self.set_pixel(xc, yc - 1, color);
            self.set_pixel(xc + 1, yc, color);
            self.set_pixel(xc - 1, yc, color);

            if let Some(fill) = fill_color {
                self.set_pixel(xc, yc, fill)
            }

            return;
        }

        self.draw_circle(xc, yc, x, y, color, fill_color);

        while y >= x {
            x += 1;

            if d > 0 {
                d = d + 4 * (x - y) + 10;
                y -= 1;
            } else {
                d = d + 4 * x + 6;
            }

            self.draw_circle(xc, yc, x, y, color, fill_color);
        }
    }
}
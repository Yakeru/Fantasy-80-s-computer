use fantasy_cpc_display_controller::color_palettes::LIGHT_GREY;

#[allow(dead_code)]
pub struct Texture {
    width: usize,
    height: usize,
    color: usize,
    columns: Vec<Vec<usize>>,
}

#[allow(dead_code)]
impl Texture {
    pub fn new(data: &[usize], width: usize, height: usize) -> Self {
        let mut columns: Vec<Vec<usize>> = Vec::new();

        for texture_row in data.chunks_exact(width) {
            for pixel in texture_row.chunks_exact(1).enumerate() {
                match columns.get_mut(pixel.0) {
                    Some(column) => {
                        column.push(pixel.1[0]);
                    }
                    None => {
                        let column: Vec<usize> = vec![pixel.1[0]];
                        columns.push(column);
                    }
                }
            }
        }

        Texture {
            width,
            height,
            color: LIGHT_GREY,
            columns,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_color(&self) -> usize {
        self.color
    }

    pub fn get_column(&self, index: usize) -> &Vec<usize> {
        &self.columns[index]
    }
}

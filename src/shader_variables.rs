use display_controller::config::{WIDTH, HEIGHT};

pub struct ShaderVariables {
    pub screen_width: f32,
    pub screen_height: f32,
    pub mode: f32,
    pub scanline_interval: f32,
    pub mask_size: f32,
    pub mask_type: f32,
    pub horiz_distortion: f32,
    pub vert_distortion: f32
}

impl ShaderVariables {
    pub fn new() -> Self {
        Self {
            screen_width: WIDTH as f32,
            screen_height: HEIGHT as f32,
            mode: 0.0,
            scanline_interval: 6.0 as f32,
            mask_size: 3.0 as f32,
            mask_type: 3.0,
            horiz_distortion: 0.0,
            vert_distortion: 0.0
        }
    }

    pub fn to_ne_bytes(&self) -> [u8; 32] {
        let whole: [u8; 32] = {
            let mut whole: [u8; 32] = [0; 32];
            let mut iterator = whole.chunks_exact_mut(4);
            iterator.next().unwrap().copy_from_slice(&self.screen_width.to_ne_bytes());
            iterator.next().unwrap().copy_from_slice(&self.screen_height.to_ne_bytes());
            iterator.next().unwrap().copy_from_slice(&self.mode.to_ne_bytes());
            iterator.next().unwrap().copy_from_slice(&self.scanline_interval.to_ne_bytes());
            iterator.next().unwrap().copy_from_slice(&self.mask_size.to_ne_bytes());
            iterator.next().unwrap().copy_from_slice(&self.mask_type.to_ne_bytes());
            iterator.next().unwrap().copy_from_slice(&self.horiz_distortion.to_ne_bytes());
            iterator.next().unwrap().copy_from_slice(&self.vert_distortion.to_ne_bytes());
            whole
        };
        whole
    }
}
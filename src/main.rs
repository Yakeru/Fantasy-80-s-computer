use winit::{
    event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::PhysicalSize
};
use winit_input_helper::WinitInputHelper;
use winit_input_helper::TextChar;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use std::time::{
    Instant
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;

const VIRTUAL_WIDTH: u32 = 426;  //426*3 = 1278 draw one black line on each side of screen for perfect *3 scale
const VIRTUAL_HEIGHT: u32 = 240; //240*4 = 960

const FPS: u128 = 32; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

const BORDER: u32 = 20;
const BKG_COLOR: u8 = 4;
const FRG_COLOR: u8 = 5;

// enum character {
//     character([u8;8]),
//     None
// }

fn character_rom(character: &char) -> [u8;8] {

    match character {
        'A' => [0x18, 0x3C, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x0],
        'B' => [0xFC, 0x66, 0x66, 0x7C, 0x66, 0x66, 0xFC, 0x0],
        'C' => [0x3C, 0x66, 0xC0, 0xC0, 0xC0, 0x66, 0x3C, 0x0],
        'D' => [0xF8, 0x6C, 0x66, 0x66, 0x66, 0x6C, 0xF8, 0x0],
        'E' => [0xFE, 0x62, 0x68, 0x78, 0x68, 0x62, 0xFE, 0x0],
        'F' => [0xFE, 0x62, 0x68, 0x78, 0x68, 0x60, 0xF0, 0x0],
        'G' => [0x3C, 0x66, 0xC0, 0xC0, 0xCE, 0xC6, 0x7E, 0x0],
        'H' => [0x66, 0x66, 0x66, 0x7E, 0x66, 0x66, 0x66, 0x0],
        'I' => [0x7E, 0x18, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x0],
        'J' => [0x1E, 0x0C, 0x0C, 0x0C, 0xCC, 0xCC, 0x78, 0x0],
        'K' => [0xE6, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0xE6, 0x0],
        'L' => [0xF0, 0x60, 0x60, 0x60, 0x62, 0x66, 0xFE, 0x0],
        'M' => [0xC6, 0xEE, 0xFE, 0xFE, 0xD6, 0xC6, 0xC6, 0x0],
        'N' => [0xC6, 0xE6, 0xF6, 0xDE, 0xCE, 0xC6, 0xC6, 0x0],
        'O' => [0x38, 0x6C, 0xC6, 0xC6, 0xC6, 0x6C, 0x38, 0x0],
        'P' => [0xFC, 0x66, 0x66, 0x78, 0x60, 0x60, 0xF0, 0x0],
        'Q' => [0x38, 0x6C, 0xC6, 0xC6, 0xDA, 0xCC, 0x76, 0x0],
        'R' => [0xFC, 0x66, 0x66, 0x7C, 0x6C, 0x66, 0xE2, 0x0],
        'S' => [0x3C, 0x66, 0x60, 0x3C, 0x06, 0x66, 0x3C, 0x0],
        'T' => [0x7E, 0x5A, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x0],
        'U' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x0],
        'V' => [0x66, 0x66, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x0],
        'W' => [0xC6, 0xC6, 0xC6, 0xD6, 0xFE, 0xEE, 0xC6, 0x0],
        'X' => [0xC6, 0x6C, 0x38, 0x38, 0x6C, 0xC6, 0xC6, 0x0],
        'Y' => [0x66, 0x66, 0x66, 0x3C, 0x18, 0x18, 0x3C, 0x0],
        'Z' => [0xFE, 0xC6, 0x8C, 0x18, 0x32, 0x66, 0xFE, 0x0],
        'a' => [0x00, 0x00, 0x78, 0x0C, 0x7C, 0xCC, 0x76, 0x0],
        'b' => [0xE0, 0xE0, 0x7C, 0x66, 0x66, 0x66, 0xBC, 0x0],
        'c' => [0x00, 0x00, 0x3C, 0x66, 0x60, 0x66, 0x3C, 0x0],
        'd' => [0x1C, 0x0C, 0x7C, 0xCC, 0xCC, 0xCC, 0x76, 0x0],
        'e' => [0x00, 0x00, 0x3C, 0x66, 0x7E, 0x60, 0x3C, 0x0],
        'f' => [0x1C, 0x36, 0x30, 0x78, 0x30, 0x30, 0x78, 0x0],
        'g' => [0x00, 0x00, 0x3E, 0x66, 0x66, 0x3E, 0x06, 0x7C],
        'h' => [0xE0, 0x60, 0x6C, 0x76, 0x66, 0x66, 0xE6, 0x0],
        'i' => [0x18, 0x00, 0x38, 0x18, 0x18, 0x18, 0x3C, 0x0],
        'j' => [0x02, 0x00, 0x0E, 0x06, 0x06, 0x66, 0x66, 0x3C],
        'k' => [0xE0, 0x60, 0x66, 0x6C, 0x78, 0x6C, 0xE6, 0x0],
        'l' => [0x38, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x0],
        'm' => [0x00, 0x00, 0x6C, 0xFE, 0xD6, 0xD6, 0xC6, 0x0],
        'n' => [0x00, 0x00, 0xD8, 0x66, 0x66, 0x66, 0x66, 0x0],
        'o' => [0x00, 0x00, 0x3C, 0x66, 0x66, 0x66, 0x3C, 0x0],
        'p' => [0x00, 0x00, 0xDC, 0x66, 0x66, 0x7C, 0x60, 0xF0],
        'q' => [0x00, 0x00, 0x76, 0xCC, 0xCC, 0x7C, 0x0C, 0x1E],
        'r' => [0x00, 0x00, 0xD8, 0x6C, 0x60, 0x60, 0xF0, 0x0],
        's' => [0x00, 0x00, 0x3C, 0x60, 0x3C, 0x06, 0x7C, 0x0],
        't' => [0x30, 0x30, 0x7C, 0x30, 0x30, 0x36, 0x1C, 0x0],
        'u' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x66, 0x3E, 0x0],
        'v' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x3C, 0x18, 0x0],
        'w' => [0x00, 0x00, 0xC6, 0xD6, 0xD6, 0xFE, 0x6C, 0x0],
        'x' => [0x00, 0x00, 0xC6, 0x6C, 0x38, 0x6C, 0xC6, 0x0],
        'y' => [0x00, 0x00, 0x66, 0x66, 0x66, 0x3E, 0x06, 0x7C],
        'z' => [0x00, 0x00, 0x7E, 0x4C, 0x18, 0x30, 0x7E, 0x00],
        ' ' => [0, 0, 0, 0, 0, 0, 0, 0],
        '-' => [0, 0, 0, 0, 0, 0, 0, 0],
        '_' => [0, 0, 0, 0, 0, 0, 0, 0],
        '=' => [0, 0, 0, 0, 0, 0, 0, 0],
        '+' => [0, 0, 0, 0, 0, 0, 0, 0],
        '/' => [0, 0, 0, 0, 0, 0, 0, 0],
        '\\' => [0, 0, 0, 0, 0, 0, 0, 0],
        '.' => [0, 0, 0, 0, 0, 0, 0, 0],
        '[' => [0, 0, 0, 0, 0, 0, 0, 0],
        ']' => [0, 0, 0, 0, 0, 0, 0, 0],
        '(' => [0, 0, 0, 0, 0, 0, 0, 0],
        ')' => [0, 0, 0, 0, 0, 0, 0, 0],
        '{' => [0, 0, 0, 0, 0, 0, 0, 0],
        '}' => [0, 0, 0, 0, 0, 0, 0, 0],
        '@' => [0, 0, 0, 0, 0, 0, 0, 0],
        '&' => [0, 0, 0, 0, 0, 0, 0, 0],
        '0' => [0, 0, 0, 0, 0, 0, 0, 0],
        '1' => [0, 0, 0, 0, 0, 0, 0, 0],
        '2' => [0, 0, 0, 0, 0, 0, 0, 0],
        '3' => [0, 0, 0, 0, 0, 0, 0, 0],
        '4' => [0, 0, 0, 0, 0, 0, 0, 0],
        '5' => [0, 0, 0, 0, 0, 0, 0, 0],
        '6' => [0, 0, 0, 0, 0, 0, 0, 0],
        '7' => [0, 0, 0, 0, 0, 0, 0, 0],
        '8' => [0, 0, 0, 0, 0, 0, 0, 0],
        '9' => [0, 0, 0, 0, 0, 0, 0, 0],
        _ => [0, 0, 0, 0, 0, 0, 0, 0]
    }
}



fn main()-> Result<(), Error> {

    //let mut color_map: [(u8, u8, u8); 32];

    let mut horizontal_multiplier: usize = 1;
    let mut vertical_multiplier: usize = 1;
    let mut crt_effet_on: bool = false;
    let crt_scanline_strength: u8 = 30;
    let crt_subpx_attenuation: u8 = 230;

    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_title("Yay, une fenêtre !")
        .with_resizable(false);

    let window = builder.build(&event_loop).expect("Window creation failed !");
    let mut input = WinitInputHelper::new();
    
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);

    let mut console_buffer: Vec<char> = Vec::new();

    let mut last_refresh: Instant = Instant::now();

    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;

        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                println!("The Escape key was pressed; stopping");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        match event {

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::ReceivedCharacter(c) => {
                    println!("{}", c);
                    console_buffer.push(c);
                }
                _ => ()
            },
            Event::MainEventsCleared => {
                // Application update code.
    
                // Queue a RedrawRequested event.
                if last_refresh.elapsed().as_millis() >= FPS {

                    let render_time: Instant = Instant::now();

                    //Draw stuff in virtual frame buffer
                    clear_frame_buffer(pixels.get_frame());
                    draw_loading_border(virtual_frame_buffer.get_frame());
                    draw_background(virtual_frame_buffer.get_frame());
                    draw_shell(&console_buffer, virtual_frame_buffer.get_frame());
                    
                    //Upscale into Pixels frame buffer
                    // if crt_effet_on {
                    //     upscale_3_by_4_with_crt_effect(virtual_frame_buffer.get_frame(), pixels.get_frame(), crt_scanline_strength, crt_subpx_attenuation);
                    // } else {
                    //     upscale_virtualfb_to_pixelfb(virtual_frame_buffer.get_frame(), pixels.get_frame(), horizontal_multiplier, vertical_multiplier);
                    // }

                    upscale_3_by_4_with_crt_effect(virtual_frame_buffer.get_frame(), pixels.get_frame(), crt_scanline_strength, crt_subpx_attenuation);
                    
                    //println!("draw time {}us", render_time.elapsed().as_micros());
                    pixels.render().expect("Pixels render oups");
                    window.request_redraw();
                    last_refresh = Instant::now();
                } 
            }
            _ => ()
        }
    });
}

fn clear_frame_buffer(frame_buffer: &mut[u8]) {
    for value in frame_buffer.chunks_exact_mut(1) {
        value[0] = 0;
    }
}

fn draw_shell(console_buffer: &Vec<char>, virtual_frame_buffer: &mut[u8]) {

    let mut x_pos = BORDER;
    let mut y_pos = BORDER;

    for c in console_buffer {

        let pic = character_rom(c);

        for row_count in 0..8 {

            let row = pic[row_count];
            let row_in_binary = &format!("{:0>8b}", row);
            let mut col_count = 0;

            for c in row_in_binary.chars() {

                match c {
                    '0' => virtual_frame_buffer[x_pos as usize + col_count + (y_pos as usize + row_count ) * VIRTUAL_WIDTH as usize] = BKG_COLOR,
                    '1' => virtual_frame_buffer[x_pos as usize + col_count + (y_pos as usize + row_count ) * VIRTUAL_WIDTH as usize] = FRG_COLOR,
                    _ => ()
                }
                col_count += 1;
            }
        }

        x_pos += 8;

        if x_pos > 50 * 8 {
            x_pos = BORDER;
            y_pos += 8;
        } 

        if y_pos > 30 * 8 {
            x_pos = BORDER;
            y_pos = BORDER;
        }
    }
}

fn draw_loading_border(frame_buffer: &mut[u8]) {
    let mut random = rand::thread_rng();
    let mut rgb_color: u8 = random.gen_range(0..8);

    let mut line_pixel_count: u32 = 0;
    let mut line_count: u32 = 0;

    for pixel in frame_buffer.chunks_exact_mut(1) {

        if line_pixel_count < BORDER || line_pixel_count > VIRTUAL_WIDTH - BORDER || line_count < BORDER || line_count > VIRTUAL_HEIGHT - BORDER {
            if (line_count % (VIRTUAL_HEIGHT/48) == 0) && line_pixel_count == 0 {
                rgb_color = random.gen_range(0..8);
            }

            pixel[0] = rgb_color;
        }
    
        line_pixel_count += 1;

        if line_pixel_count == VIRTUAL_WIDTH {
            line_count += 1;
            line_pixel_count = 0;
        }
    }
}

fn draw_background(frame_buffer: &mut[u8]) {

    let mut screen_pixel_count: u32 = 0;
    let mut line_count: u32 = 0;

    for pixel in frame_buffer.chunks_exact_mut(1) {

        if screen_pixel_count >= BORDER && screen_pixel_count <= VIRTUAL_WIDTH - BORDER && line_count >= BORDER && line_count <= VIRTUAL_HEIGHT - BORDER {
            pixel[0] = BKG_COLOR;
        }

        screen_pixel_count += 1;

        if screen_pixel_count == VIRTUAL_WIDTH {
            line_count += 1;
            screen_pixel_count = 0;
        }
    }
}

// struct Sprite {
//     pos_x: u32,
//     pos_y: u32,
//     size_x: u32,
//     size_y: u32,
//     visible: bool,
//     image: Vec<u8>
// }

// impl Sprite {

//     fn new() -> Sprite { 

//         let size: u32 = 8 * 8;
//         let mut new_image = Vec::new();

//         for _value in 0..size {
//             new_image.push(0);
//         }

//         Sprite {
//             pos_x: 0,
//             pos_y: 0,
//             size_x: 8,
//             size_y: 8,
//             visible: false,
//             image: new_image
//         }
//     }
// }

struct VirtualFrameBuffer {
    width: u32,
    height: u32,
    frame: Vec<u8>,
}

impl VirtualFrameBuffer {
    fn new(width: u32, height: u32) -> VirtualFrameBuffer {
        let size: u32 = width * height;
        let mut virtual_frame_buffer = Vec::new();

        for _value in 0..size {
            virtual_frame_buffer.push(0);
        }

        VirtualFrameBuffer {
            width: width,
            height: height,
            frame: virtual_frame_buffer
        }
    }

    fn get_frame(&mut self) -> &mut [u8] {
        return &mut self.frame;
    }
}

fn upscale_virtualfb_to_pixelfb(virtual_fb: &mut[u8], pixels_frame: &mut[u8], integer_width_multiplier: usize, integer_height_multiplier: usize) {
    
    let mut virt_line_pixel_counter: usize = 0;
    let mut virt_line_counter: usize = 0;
    let pixels_sub_pixel_count = 4;

    for pixel in virtual_fb {

        //Temporary color index to RGB mapping
        let mut rgb: (u8, u8, u8) = (0, 0, 0);
        match pixel {
            0 => {
                rgb.0 = 0;
                rgb.1 = 0;
                rgb.2 = 0;
            },
            1 => {
                rgb.0 = 254;
                rgb.1 = 254;
                rgb.2 = 254;
            },
            2 => {
                rgb.0 = 254;
                rgb.1 = 0;
                rgb.2 = 0;
            },
            3 => {
                rgb.0 = 0;
                rgb.1 = 254;
                rgb.2 = 0;
            },
            4 => {
                rgb.0 = 0;
                rgb.1 = 0;
                rgb.2 = 254;
            },
            5 => {
                rgb.0 = 254;
                rgb.1 = 254;
                rgb.2 = 0;
            },
            6 => {
                rgb.0 = 0;
                rgb.1 = 254;
                rgb.2 = 254;
            },
            7 => {
                rgb.0 = 254;
                rgb.1 = 0;
                rgb.2 = 254;
            },
            8.. => {
                rgb.0 = 0;
                rgb.1 = 0;
                rgb.2 = 0;
            }
        }

        //Offset between virtual frame buffer and pixel's frame buffer
        //if scaling is applied, it represents the offset between virtual frame buffer's pixel and
        //pixel's top-left corner of scalled pixel
        let global_offset = pixels_sub_pixel_count * virt_line_pixel_counter * integer_width_multiplier 
        + WIDTH as usize * pixels_sub_pixel_count * virt_line_counter * integer_height_multiplier;
        
        for horizontal_copy in 0..integer_width_multiplier {
            for vertical_copy in 0..integer_height_multiplier {
                let scaling_offset:usize = pixels_sub_pixel_count * horizontal_copy + WIDTH as usize * pixels_sub_pixel_count * vertical_copy;
                let final_offset: usize = global_offset + scaling_offset;
                pixels_frame[0 + final_offset] = rgb.0;
                pixels_frame[1 + final_offset] = rgb.1;
                pixels_frame[2 + final_offset] = rgb.2;
                pixels_frame[3 + final_offset] = 254;
            }
        }

        virt_line_pixel_counter += 1;
        if virt_line_pixel_counter == VIRTUAL_WIDTH as usize {
            virt_line_pixel_counter = 0;
            virt_line_counter += 1;
        }
    }
}

fn upscale_3_by_4_with_crt_effect(virtual_fb: &mut[u8], pixels_frame: &mut[u8], scan_strength: u8, sub_pixel_masking: u8) {
    let mut virt_line_pixel_counter: usize = 0;
    let mut virt_line_counter: usize = 0;
    let pixels_sub_pixel_count = 4;

    for pixel in virtual_fb {

        //Temporary color index to RGB mapping
        let mut rgb: (u8, u8, u8) = (0, 0, 0);
        match pixel {
            0 => {
                rgb.0 = 0;
                rgb.1 = 0;
                rgb.2 = 0;
            },
            1 => {
                rgb.0 = 254;
                rgb.1 = 254;
                rgb.2 = 254;
            },
            2 => {
                rgb.0 = 254;
                rgb.1 = 0;
                rgb.2 = 0;
            },
            3 => {
                rgb.0 = 0;
                rgb.1 = 254;
                rgb.2 = 0;
            },
            4 => {
                rgb.0 = 0;
                rgb.1 = 0;
                rgb.2 = 254;
            },
            5 => {
                rgb.0 = 254;
                rgb.1 = 254;
                rgb.2 = 0;
            },
            6 => {
                rgb.0 = 0;
                rgb.1 = 254;
                rgb.2 = 254;
            },
            7 => {
                rgb.0 = 254;
                rgb.1 = 0;
                rgb.2 = 254;
            },
            8.. => {
                rgb.0 = 0;
                rgb.1 = 0;
                rgb.2 = 0;
            }
        }

        //Offset between virtual frame buffer and pixel's frame buffer
        //if scaling is applied, it represents the offset between virtual frame buffer's pixel and
        //pixel's top-left corner of scalled pixel
        let global_offset = pixels_sub_pixel_count * virt_line_pixel_counter * 3 
        + WIDTH as usize * pixels_sub_pixel_count * virt_line_counter * 4;
        
        for horizontal_copy in 0..3 {
            for vertical_copy in 0..4 {
                let scaling_offset: usize = pixels_sub_pixel_count * horizontal_copy + WIDTH as usize * pixels_sub_pixel_count * vertical_copy;
                let final_offset: usize = global_offset + scaling_offset;
                let mut final_rgb: (u8, u8, u8) = rgb;

                match horizontal_copy {
                    0 => {
                        if final_rgb.1 < sub_pixel_masking {final_rgb.1 = 0} else {final_rgb.1 -= sub_pixel_masking};
                        if final_rgb.2 < sub_pixel_masking {final_rgb.2 = 0} else {final_rgb.2 -= sub_pixel_masking};
                    },
                    1 => {
                        if final_rgb.0 < sub_pixel_masking {final_rgb.0 = 0} else {final_rgb.0 -= sub_pixel_masking};
                        if final_rgb.2 < sub_pixel_masking {final_rgb.2 = 0} else {final_rgb.2 -= sub_pixel_masking};
                    },
                    2 => {
                        if final_rgb.0 < sub_pixel_masking {final_rgb.0 = 0} else {final_rgb.0 -= sub_pixel_masking};
                        if final_rgb.1 < sub_pixel_masking {final_rgb.1 = 0} else {final_rgb.1 -= sub_pixel_masking};
                    },
                    _ => {}
                }

                if vertical_copy == 3 {
                    if final_rgb.0 < scan_strength {final_rgb.0 = 0} else {final_rgb.0 -= scan_strength};
                    if final_rgb.1 < scan_strength {final_rgb.1 = 0} else {final_rgb.1 -= scan_strength};
                    if final_rgb.2 < scan_strength {final_rgb.2 = 0} else {final_rgb.2 -= scan_strength};
                } 

                pixels_frame[0 + final_offset] = final_rgb.0;
                pixels_frame[1 + final_offset] = final_rgb.1;
                pixels_frame[2 + final_offset] = final_rgb.2;
                pixels_frame[3 + final_offset] = 254;
                
            }
        }

        virt_line_pixel_counter += 1;
        if virt_line_pixel_counter == VIRTUAL_WIDTH as usize {
            virt_line_pixel_counter = 0;
            virt_line_counter += 1;
        }
    }
}

struct color {
    index: u8,
    r: u8,
    g: u8,
    b: u8,
    name: String
}

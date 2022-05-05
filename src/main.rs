use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::PhysicalSize
};
use winit_input_helper::WinitInputHelper;
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
    //draw_test_grid(virtual_frame_buffer.get_frame());

    let mut last_refresh: Instant = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
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

        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                println!("The Escape key was pressed; stopping");
                return;
            }

            if input.key_pressed(VirtualKeyCode::Right) {
                horizontal_multiplier += 1;
                if horizontal_multiplier > 3 {horizontal_multiplier = 3}
                return;
            }

            if input.key_pressed(VirtualKeyCode::Left) {
                horizontal_multiplier -= 1;
                if horizontal_multiplier < 1 {horizontal_multiplier = 1}
                return;
            }

            if input.key_pressed(VirtualKeyCode::Down) {
                vertical_multiplier += 1;
                if vertical_multiplier > 4 {vertical_multiplier = 4}
                return;
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                vertical_multiplier -= 1;
                if vertical_multiplier < 1 {vertical_multiplier = 1}
                return;
            }

            if input.key_pressed(VirtualKeyCode::Return) {
                crt_effet_on = !crt_effet_on;
                return;
            }
        }
    });
}

fn clear_frame_buffer(frame_buffer: &mut[u8]) {
    for value in frame_buffer.chunks_exact_mut(1) {
        value[0] = 0;
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

fn draw_test_grid(virtual_fb: &mut[u8]) {
    let mut screen_pixel_count: u32 = 0;
    let mut line_count: u32 = 0;

    for pixel in virtual_fb.chunks_exact_mut(1) {

        if screen_pixel_count % 3 == 0 {
            pixel[0] = 2;
        }

        if screen_pixel_count % 7 == 0 {
            pixel[0] = 3;
        }

        if screen_pixel_count % 11 == 0 {
            pixel[0] = 4;
        }

        if screen_pixel_count % 13 == 0 {
            pixel[0] = 1;
        }

        if line_count % 3 == 0 {
            pixel[0] = 2;
        }

        if line_count % 7 == 0 {
            pixel[0] = 3;
        }

        if line_count % 11 == 0 {
            pixel[0] = 4;
        }

        if line_count % 13 == 0 {
            pixel[0] = 1;
        }

        if line_count == 0 {
            pixel[0] = 1;
        }

        if screen_pixel_count == 0 {
            pixel[0] = 1;
        }

        if screen_pixel_count == VIRTUAL_WIDTH - 1 {
            pixel[0] = 1;
        }

        if line_count == VIRTUAL_HEIGHT - 1 {
            pixel[0] = 1;
        }

        screen_pixel_count += 1;

        if screen_pixel_count == VIRTUAL_WIDTH {
            line_count += 1;
            screen_pixel_count = 0;
        }
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


    //black(u8, u8,u8,u8, String) = (0, 0, 0, 0, "black"),
    //dark_blue(u8, u8,u8,u8, String) = (0, 0, 0, 0, "black")
    // 2 		#7E2553 	126, 37, 83 	dark-purple
    // 3 		#008751 	0, 135, 81 	dark-green
    // 4 		#AB5236 	171, 82, 54 	brown
    // 5 		#5F574F 	95, 87, 79 	dark-grey
    // 6 		#C2C3C7 	194, 195, 199 	light-grey
    // 7 		#FFF1E8 	255, 241, 232 	white
    // 8 		#FF004D 	255, 0, 77 	red
    // 9 		#FFA300 	255, 163, 0 	orange
    // 10 		#FFEC27 	255, 236, 39 	yellow
    // 11 		#00E436 	0, 228, 54 	green
    // 12 		#29ADFF 	41, 173, 255 	blue
    // 13 		#83769C 	131, 118, 156 	lavender
    // 14 		#FF77A8 	255, 119, 168 	pink
    // 15 		#FFCCAA 	255, 204, 170 	light-peach

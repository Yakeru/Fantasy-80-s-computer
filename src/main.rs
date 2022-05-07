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

use crate::characters::rom;
mod characters;

use crate::virtual_frame_buffer::VirtualFrameBuffer;
mod virtual_frame_buffer;

use crate::sprite::Sprite;
mod sprite;

use crate::shell::Shell;
mod shell;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;

const VIRTUAL_WIDTH: u32 = 426;  //426*3 = 1278 draw one black line on each side of screen for perfect *3 scale
const VIRTUAL_HEIGHT: u32 = 240; //240*4 = 960

const FPS: u128 = 16; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

const BKG_COLOR: u8 = 4;
const FRG_COLOR: u8 = 5;

const SHELL_BUFFER_SIZE: usize = 1000;
const TEXT_COLUMNS: u8 = 40;
const TEXT_ROWS: u8 = 25;

fn main()-> Result<(), Error> {

    let crt_scanline_strength: u8 = 35;
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
    let mut shell: Shell = Shell::new(1000, TEXT_COLUMNS, TEXT_ROWS);

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
                    shell.send_character_to_shell(c);
                }
                _ => ()
            },
            Event::MainEventsCleared => {
                // Application update code.
    
                // Queue a RedrawRequested event.
                if last_refresh.elapsed().as_millis() >= FPS {

                    //let render_time: Instant = Instant::now();

                    //Draw stuff in virtual frame buffer
                    virtual_frame_buffer.clear_frame_buffer(BKG_COLOR);
                    draw_loading_border(virtual_frame_buffer.get_frame(), 20, 20);
                    draw_shell_to_framebuffer(&shell, &mut virtual_frame_buffer);
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

fn draw_loading_border(frame_buffer: &mut[u8], vert_size: u8, horiz_size: u8) {
    let mut random = rand::thread_rng();
    let mut rgb_color: u8 = random.gen_range(0..8);

    let mut line_pixel_count: u32 = 0;
    let mut line_count: u32 = 0;

    for pixel in frame_buffer.chunks_exact_mut(1) {

        if line_pixel_count < horiz_size as u32 || line_pixel_count > VIRTUAL_WIDTH - horiz_size as u32 || line_count < vert_size as u32 || line_count > VIRTUAL_HEIGHT - vert_size as u32 {
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

pub fn draw_shell_to_framebuffer(shell: &Shell, virtual_frame_buffer: &mut VirtualFrameBuffer) {

    let horizontal_border: u32 = (virtual_frame_buffer.get_width() as u32 - shell.get_nb_columns() as u32 * shell::CHARACTER_WIDTH as u32) / 2;
    let vertical_border: u32 = (virtual_frame_buffer.get_height() - shell.get_nb_rows() as u32 * shell::CHARACTER_HEIGHT as u32) / 2;

    let mut x_pos = horizontal_border;
    let mut y_pos = vertical_border;

    let mut shell_row_count = 0;
    let mut shell_col_count = 0;

    for c in shell.get_buffer() {

        let pic = rom(c);

        for row_count in 0..8 {

            let row = pic[row_count];
            let row_in_binary = &format!("{:0>8b}", row);
            let mut character_sprite_col_count = 0;

            for c in row_in_binary.chars() {

                match c {
                    '0' => virtual_frame_buffer.get_frame()[x_pos as usize + character_sprite_col_count + (y_pos as usize + row_count ) * VIRTUAL_WIDTH as usize] = BKG_COLOR,
                    '1' => virtual_frame_buffer.get_frame()[x_pos as usize + character_sprite_col_count + (y_pos as usize + row_count ) * VIRTUAL_WIDTH as usize] = FRG_COLOR,
                    _ => ()
                }
                character_sprite_col_count += 1;
            }
        }

        shell_col_count += 1;
        x_pos += shell::CHARACTER_WIDTH as u32;

        if shell_col_count == shell.get_nb_columns() as u32 {
            shell_col_count = 0;
            shell_row_count += 1;
            x_pos = horizontal_border;
            y_pos += shell::CHARACTER_HEIGHT as u32;
        } 

        if shell_row_count == shell.get_nb_rows() as u32 {
            shell_col_count = 0;
            shell_row_count = 0;
            x_pos = horizontal_border;
            y_pos = vertical_border;
        }

        //Draw cursor
        let cursor = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        for row_count in 0..8 {

            let row = cursor[row_count];
            let row_in_binary = &format!("{:0>8b}", row);
            let mut col_count = 0;

            for c in row_in_binary.chars() {

                match c {
                    '0' => virtual_frame_buffer.get_frame()[x_pos as usize + col_count + (y_pos as usize + row_count ) * VIRTUAL_WIDTH as usize] = BKG_COLOR,
                    '1' => virtual_frame_buffer.get_frame()[x_pos as usize + col_count + (y_pos as usize + row_count ) * VIRTUAL_WIDTH as usize] = FRG_COLOR,
                    _ => ()
                }
                col_count += 1;
            }
        }
    }
}

// fn upscale_virtualfb_to_pixelfb(virtual_fb: &mut[u8], pixels_frame: &mut[u8], integer_width_multiplier: usize, integer_height_multiplier: usize) {
    
//     let mut virt_line_pixel_counter: usize = 0;
//     let mut virt_line_counter: usize = 0;
//     let pixels_sub_pixel_count = 4;

//     for pixel in virtual_fb {

//         //Temporary color index to RGB mapping
//         let mut rgb: (u8, u8, u8) = (0, 0, 0);
//         match pixel {
//             0 => {
//                 rgb.0 = 0;
//                 rgb.1 = 0;
//                 rgb.2 = 0;
//             },
//             1 => {
//                 rgb.0 = 254;
//                 rgb.1 = 254;
//                 rgb.2 = 254;
//             },
//             2 => {
//                 rgb.0 = 254;
//                 rgb.1 = 0;
//                 rgb.2 = 0;
//             },
//             3 => {
//                 rgb.0 = 0;
//                 rgb.1 = 254;
//                 rgb.2 = 0;
//             },
//             4 => {
//                 rgb.0 = 0;
//                 rgb.1 = 0;
//                 rgb.2 = 254;
//             },
//             5 => {
//                 rgb.0 = 254;
//                 rgb.1 = 254;
//                 rgb.2 = 0;
//             },
//             6 => {
//                 rgb.0 = 0;
//                 rgb.1 = 254;
//                 rgb.2 = 254;
//             },
//             7 => {
//                 rgb.0 = 254;
//                 rgb.1 = 0;
//                 rgb.2 = 254;
//             },
//             8.. => {
//                 rgb.0 = 0;
//                 rgb.1 = 0;
//                 rgb.2 = 0;
//             }
//         }

//         //Offset between virtual frame buffer and pixel's frame buffer
//         //if scaling is applied, it represents the offset between virtual frame buffer's pixel and
//         //pixel's top-left corner of scalled pixel
//         let global_offset = pixels_sub_pixel_count * virt_line_pixel_counter * integer_width_multiplier 
//         + WIDTH as usize * pixels_sub_pixel_count * virt_line_counter * integer_height_multiplier;
        
//         for horizontal_copy in 0..integer_width_multiplier {
//             for vertical_copy in 0..integer_height_multiplier {
//                 let scaling_offset:usize = pixels_sub_pixel_count * horizontal_copy + WIDTH as usize * pixels_sub_pixel_count * vertical_copy;
//                 let final_offset: usize = global_offset + scaling_offset;
//                 pixels_frame[0 + final_offset] = rgb.0;
//                 pixels_frame[1 + final_offset] = rgb.1;
//                 pixels_frame[2 + final_offset] = rgb.2;
//                 pixels_frame[3 + final_offset] = 254;
//             }
//         }

//         virt_line_pixel_counter += 1;
//         if virt_line_pixel_counter == VIRTUAL_WIDTH as usize {
//             virt_line_pixel_counter = 0;
//             virt_line_counter += 1;
//         }
//     }
// }

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
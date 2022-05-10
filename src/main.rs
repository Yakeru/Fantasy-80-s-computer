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
use std::io::{self, Write};

mod characters_rom;
mod text_layer;
mod virtual_frame_buffer;
mod color_palettes;

use crate::text_layer::{TextLayer, TextLayerRenderer};
use crate::virtual_frame_buffer::{VirtualFrameBuffer, CrtEffectRenderer};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;

const VIRTUAL_WIDTH: u32 = 426;  //426*3 = 1278 draw one black line on each side of screen for perfectly centered *3 scale
const VIRTUAL_HEIGHT: u32 = 240; //240*4 = 960

const FPS: u128 = 16; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

const SPLASH_1: &str = "************** FANTASY CPC *************";
const SPLASH_2: &str = "*               ROM v0.1               *";
const SPLASH_3: &str = "*        Damien Torreilles 2022        *";
const SPLASH_4: &str = "****************************************";
const SPLASH_5: &str = "Ready. Type 'help' for command list.    ";
const SPLASH_6: &str = "                                        ";

const DEFAULT_BKG_COLOR: u8 = 4;
const DEFAULT_COLOR: u8 = 5;
const TEXT_COLUMNS: u8 = 40;
const TEXT_ROWS: u8 = 25;

fn main()-> Result<(), Error> {

    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_title("Yay, une fenêtre !")
        .with_resizable(false);

    let window = builder.build(&event_loop).expect("Window creation failed !");
    let mut input = WinitInputHelper::new();
    
    //TODO in the future : have a surface texture upscale the buffer instead of upscaling by hand pixel by pixel ...
    //and learn to do shader for the CRT effect.
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut text_color = DEFAULT_COLOR;
    let mut text_bkg_color = DEFAULT_BKG_COLOR;
    let mut virtual_text_layer_buffer = TextLayer::new(TEXT_COLUMNS as u32, TEXT_ROWS as u32);
    let text_renderer = TextLayerRenderer::new(TEXT_COLUMNS as u32, TEXT_ROWS as u32, VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
    let crt_renderer: CrtEffectRenderer = CrtEffectRenderer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, WIDTH, HEIGHT);

    let mut last_refresh: Instant = Instant::now();

    //Boot
    virtual_text_layer_buffer.push_string(SPLASH_1, text_color, text_bkg_color, false);
    virtual_text_layer_buffer.push_string(SPLASH_2, text_color, text_bkg_color, false);
    virtual_text_layer_buffer.push_string(SPLASH_3, text_color, text_bkg_color, false);
    virtual_text_layer_buffer.push_string(SPLASH_4, text_color, text_bkg_color, false);
    virtual_text_layer_buffer.push_string(SPLASH_5, text_color, text_bkg_color, false);
    virtual_text_layer_buffer.push_string(SPLASH_6, text_color, text_bkg_color, false);

    virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);

    let mut command: Vec<char> = Vec::new();

    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;

        if input.update(&event) {
            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                println!("The Escape key was pressed; stopping");
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_released(VirtualKeyCode::Left) {
                if text_color == 7 {text_color = 0} else {text_color += 1}
                virtual_text_layer_buffer.pop_char();
                virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
            }

            if input.key_released(VirtualKeyCode::Right) {
                if text_color == 0 {text_color = 7} else {text_color -= 1}
                virtual_text_layer_buffer.pop_char();
                virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
            }

            if input.key_released(VirtualKeyCode::Up) {
                if text_bkg_color == 7 {text_bkg_color = 0} else {text_bkg_color += 1}
                virtual_text_layer_buffer.pop_char();
                virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
            }

            if input.key_released(VirtualKeyCode::Down) {
                if text_bkg_color == 0 {text_bkg_color = 7} else {text_bkg_color -= 1}
                virtual_text_layer_buffer.pop_char();
                virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
            }

            if input.key_released(VirtualKeyCode::PageUp) {
                virtual_text_layer_buffer.scroll_up();

                if virtual_text_layer_buffer.get_characters().len() == 0 {
                    virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
                }
            }
        }

        match event {

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::ReceivedCharacter(c) => {
                    
                    // print!("{} ", c as u8);
                    // io::stdout().flush().unwrap();

                    if c == 8 as char {
                        //8 is Backspace 
                        virtual_text_layer_buffer.pop_char(); //delete cursor
                        virtual_text_layer_buffer.pop_char(); //delete a char
                        virtual_text_layer_buffer.pop_all_none();
                        virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false); //re insert cursor
                        command.pop();
                    } else if c == 13 as char {
                        //13 is Enter
                        virtual_text_layer_buffer.pop_char(); //delete cursor
                        virtual_text_layer_buffer.push_char('\n', text_color, text_bkg_color, false); //re insert cursor
                        //push enough None characters to fill line and go to next
                        let reminder = virtual_text_layer_buffer.get_characters().len() % TEXT_COLUMNS as usize;
                        for _i in 0..(TEXT_COLUMNS as usize - reminder) {
                            virtual_text_layer_buffer.push_character(None);
                        }

                        //re insert cursor
                        virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
                        
                        //Interpret line content as command
                        let mut string_command: String = command.iter().cloned().collect();
                        string_command = string_command.trim().to_lowercase();
                        println!("Command: '{}'", string_command.trim().to_lowercase());
                        command.clear();

                        if string_command == "help" {
                            virtual_text_layer_buffer.pop_char();
                            virtual_text_layer_buffer.push_string("Help is on the way !                    ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            virtual_text_layer_buffer.push_string("'clear'", 2, DEFAULT_BKG_COLOR, false);
                            virtual_text_layer_buffer.push_string(" clears the screen.              ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            virtual_text_layer_buffer.push_string("'quit'", 2, DEFAULT_BKG_COLOR, false);
                            virtual_text_layer_buffer.push_string(" or ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            virtual_text_layer_buffer.push_string("'exit'", 2, DEFAULT_BKG_COLOR, false);
                            virtual_text_layer_buffer.push_string(" shuts down computer.   ", DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
                            virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
                        }

                        if string_command == "clear" {
                            virtual_text_layer_buffer.clear();
                            virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false);
                        }

                        if string_command == "quit" || string_command == "exit"{
                            println!("Command 'quit' or 'exit' received; stopping");
                            *control_flow = ControlFlow::Exit
                        }

                    } else {

                        if virtual_text_layer_buffer.get_characters().len() >= TEXT_COLUMNS as usize * (TEXT_ROWS as usize - 1) {
                            virtual_text_layer_buffer.scroll_up();
                        }
                        virtual_text_layer_buffer.pop_char(); //delete cursor
                        virtual_text_layer_buffer.push_char(c, text_color, text_bkg_color, false); //push new char
                        virtual_text_layer_buffer.push_char('_', text_color, text_bkg_color, false); //re insert cursor
                        command.push(c);
                    }
                }
                _ => ()
            },
            Event::MainEventsCleared => {
                // Application update code.
    
                // Queue a RedrawRequested event.
                if last_refresh.elapsed().as_millis() >= FPS {

                    //let render_time: Instant = Instant::now();

                    //Draw stuff in virtual frame buffer
                    virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
                    //draw_loading_border(virtual_frame_buffer.get_frame(), 20, 20);
                    text_renderer.render(&virtual_text_layer_buffer, &mut virtual_frame_buffer);

                    //Render virtual frame buffer to pixels's frame buffer
                    crt_renderer.render(virtual_frame_buffer.get_frame(), pixels.get_frame());
                    
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
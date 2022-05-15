use winit::{
    event::{Event, WindowEvent, VirtualKeyCode, DeviceEvent, ElementState, ModifiersState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::PhysicalSize
};
use winit_input_helper::WinitInputHelper;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use std::time::{
    Instant, Duration
};
use std::io::{self, Write};

mod characters_rom;
mod text_layer;
mod virtual_frame_buffer;
mod color_palettes;
mod process;
mod shell;
mod text_edit;
mod sprite;
mod sprite_editor;

use crate::virtual_frame_buffer::{VirtualFrameBuffer, CrtEffectRenderer};
use crate::process::*;
use crate::shell::*;
use crate::sprite::Sprite;

const WIDTH: usize = 1280;
const HEIGHT: usize = 960;

const FPS: u64 = 16; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

const DEFAULT_BKG_COLOR: u8 = 28;
const DEFAULT_COLOR: u8 = 28;
const TEXT_COLUMNS: usize = 40;
const TEXT_ROWS: usize = 30;

const VIRTUAL_WIDTH: usize = 426;  // 426*3 = 1278 draw one black line on each side of screen for perfectly centered *3 scale
const VIRTUAL_HEIGHT: usize = 320; // 320*3 = 960

const SPLASH_1: &str = "************* FANTASY CPC *************";
const SPLASH_2: &str = "*              ROM v0.1               *";
const SPLASH_3: &str = "*       Damien Torreilles 2022        *";
const SPLASH_4: &str = "***************************************";

fn main()-> Result<(), Error> {

    //winit init and setup
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(WIDTH as i32, HEIGHT as i32))
        .with_title("Yay, une fenêtre !")
        .with_resizable(false);
    let window = builder.build(&event_loop).expect("Window creation failed !");
    let mut input = WinitInputHelper::new();
    let mut modifiers = ModifiersState::default();

    //window.set_cursor_grab(true).unwrap();
    //window.set_cursor_visible(false);
    
    //Pixels frame buffer init and setup
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    //Custom intermediate frame buffer
    //Has 1/3 the horizontal resolution and 1/3 the vertical resoluton of pixels surface texture ans winit window size.
    //The virtual frame buffer has a text layer, sprite lists, background layers and tiles layers that can be accessed
    //by Processes (structs implemeting "process") to build their image.
    //Its rendere combines all the layers in its frame to produce the complete image.
    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, TEXT_COLUMNS, TEXT_ROWS);

    //The crt renderer takes the virtual frame buffers's frame, upscales it 3 times in X and Y to matche the pixcel's frame and winow size,
    //then applyes an effect to evoke CRT sub-pixels and scanlines.
    //The upscaled and "crt'ed" image is then pushed into pixel's frame for final render.
    let crt_renderer: CrtEffectRenderer = CrtEffectRenderer::new(WIDTH, HEIGHT);

    //Init Shell
    //The Shell is a process that interprets the user commands to manage other processes.
    //It is launched at startup, the winit event loop will update and render the shell by default if
    //no other process is running or has the focus.
    //It manages the start, stop, render, update of all the other processes.
    //It is always updated in the event loop event if another process has the focus (updated and rendered) 
    let mut shell = Shell::new();
    shell.set_state(true, true);

    // let mut mouse_sprite: Sprite = Sprite::new_from_file(String::from("mouse"), &String::from("./resources/sprites/sprite1.txt"));
    // mouse_sprite.pos_x = VIRTUAL_WIDTH / 2;
    // mouse_sprite.pos_y = VIRTUAL_HEIGHT / 2;
    // virtual_frame_buffer.get_sprites().push(mouse_sprite);

    //Variables used to collect all the events relevent to the shell and processes occuring during a loop.
    //Once all the vents have been cleared, they are sent to the shell for its update.
    let mut key_released: Option<VirtualKeyCode> = None;
    let mut key_pressed_os: Option<VirtualKeyCode> = None;
    let mut char_received: Option<char> = None;
    let mut mouse_move_delta: (f64, f64) = (0.0, 0.0);

    //Push the splash screen to the text layer
    virtual_frame_buffer.get_text_layer().push_string_line(SPLASH_1, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
    virtual_frame_buffer.get_text_layer().push_string_line(SPLASH_2, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
    virtual_frame_buffer.get_text_layer().push_string_line(SPLASH_3, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);
    virtual_frame_buffer.get_text_layer().push_string_line(SPLASH_4, DEFAULT_COLOR, DEFAULT_BKG_COLOR, false);

    //The event loop here can be considered as a bios rom + terminal
    //it gathers all the keyborad inputs and sends them to the shell, the shell interprets them.
    //it always runs the shell and gives it the focus if no other app is running.
    event_loop.run(move |event, _, control_flow| {

        //Control_flow::Poll used 100% of one CPU core (In Windows 10 at least)
        //WaitUntil polls every "const FPS" ms instead: droped CPU usage from 20% to 4%.
        //The whole program loops (updates and draws) at "const FPS" fps.
        let refresh_timer: Instant = Instant::now().checked_add(Duration::from_millis(FPS)).unwrap();
        *control_flow = ControlFlow::WaitUntil(refresh_timer);

        if input.update(&event) {
            if input.key_pressed_os(VirtualKeyCode::Escape) || input.quit() {
                key_pressed_os = Some(VirtualKeyCode::Escape);
            }

            if input.key_pressed_os(VirtualKeyCode::Left) {
                key_pressed_os = Some(VirtualKeyCode::Left);
            }

            if input.key_pressed_os(VirtualKeyCode::Right) {
                key_pressed_os = Some(VirtualKeyCode::Right);
            }

            if input.key_pressed_os(VirtualKeyCode::Up) {
                key_pressed_os = Some(VirtualKeyCode::Up);
            }

            if input.key_pressed_os(VirtualKeyCode::Down) {
                key_pressed_os = Some(VirtualKeyCode::Down);
            }

            if input.key_pressed_os(VirtualKeyCode::PageUp) {
                key_pressed_os = Some(VirtualKeyCode::PageUp);
            }

            if input.key_released(VirtualKeyCode::Escape) || input.quit() {
                key_released = Some(VirtualKeyCode::Escape);
            }

            if input.key_released(VirtualKeyCode::Left) {
                key_released = Some(VirtualKeyCode::Left);
            }

            if input.key_released(VirtualKeyCode::Right) {
                key_released = Some(VirtualKeyCode::Right);
            }

            if input.key_released(VirtualKeyCode::Up) {
                key_released = Some(VirtualKeyCode::Up);
            }

            if input.key_released(VirtualKeyCode::Down) {
                key_released = Some(VirtualKeyCode::Down);
            }

            if input.key_released(VirtualKeyCode::PageUp) {
                key_released = Some(VirtualKeyCode::PageUp);
            }
        }

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    char_received = None;
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::ReceivedCharacter(c) => {
                    char_received = Some(c);
                }
                _ => {
                    char_received = None;
                }
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    mouse_move_delta = delta;
                }
                DeviceEvent::Button { button, state } => match state {
                    ElementState::Pressed => println!("mouse button {} pressed", button),
                    ElementState::Released => println!("mouse button {} released", button),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                
                //Updating the shell
                let flow = shell.update(char_received, key_pressed_os, key_released);                
                match flow {
                    Some(flow) => {
                        *control_flow = flow;
                    }
                    None => ()
                }

                //Render
                shell.draw(&mut virtual_frame_buffer);
                virtual_frame_buffer.render();
                crt_renderer.render(&virtual_frame_buffer, pixels.get_frame());
                pixels.render().expect("Pixels render oups");
                window.request_redraw();

                //Reset input buffers for next loop
                char_received = None;
                key_pressed_os = None;
                key_released = None;
                mouse_move_delta.0 = 0.0;
                mouse_move_delta.1 = 0.0;
            }
            _ => ()
        }
    });
}

fn draw_loading_border(frame_buffer: &mut[u8], vert_size: usize, horiz_size: usize) {
    let mut random = rand::thread_rng();
    let mut rgb_color: u8 = random.gen_range(0..32);

    let mut line_pixel_count: usize = 0;
    let mut line_count: usize = 0;
    let mut band_count: u8 = 0;
    let mut band: u8 = random.gen_range(0..20) + 4;

    for pixel in frame_buffer.chunks_exact_mut(1) {

        if line_pixel_count < horiz_size || line_pixel_count > VIRTUAL_WIDTH - horiz_size || line_count < vert_size || line_count > VIRTUAL_HEIGHT - vert_size {
            if band_count >= band {
                rgb_color = random.gen_range(0..32);
                band_count = 0;
                band  = random.gen_range(0..20) + 4;
            }

            pixel[0] = rgb_color;
        }
    
        line_pixel_count += 1;


        if line_pixel_count == VIRTUAL_WIDTH {
            band_count += 1;
            line_count += 1;
            line_pixel_count = 0;
        }
    }
}
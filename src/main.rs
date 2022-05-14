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
mod cli;
mod text_edit;
mod sprite;
mod sprite_editor;

use crate::virtual_frame_buffer::{VirtualFrameBuffer, CrtEffectRenderer};
use crate::process::*;
use crate::cli::*;
use crate::text_edit::*;
use crate::sprite::Sprite;
use crate::sprite_editor::*;

const WIDTH: usize = 1280;
const HEIGHT: usize = 960;

const FPS: u64 = 16; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

const DEFAULT_BKG_COLOR: u8 = 4;
const TEXT_COLUMNS: usize = 40;
const TEXT_ROWS: usize = 30;

const VIRTUAL_WIDTH: usize = 426;  // 426*3 = 1278 draw one black line on each side of screen for perfectly centered *3 scale
const VIRTUAL_HEIGHT: usize = 320; // 320*3 = 960

fn main()-> Result<(), Error> {

    let event_loop = EventLoop::new();
    let event_loop_proxy = event_loop.create_proxy();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(WIDTH as i32, HEIGHT as i32))
        .with_title("Yay, une fenêtre !")
        .with_resizable(false);

    let window = builder.build(&event_loop).expect("Window creation failed !");
    let mut input = WinitInputHelper::new();

    window.set_cursor_grab(true).unwrap();
    window.set_cursor_visible(false);

    let mut modifiers = ModifiersState::default();
    
    //TODO in the future : have a surface texture upscale the buffer instead of upscaling by hand pixel by pixel ...
    //and learn to do shaders for the CRT effect.
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, TEXT_COLUMNS, TEXT_ROWS);
    let crt_renderer: CrtEffectRenderer = CrtEffectRenderer::new(WIDTH, HEIGHT);

    let mut last_refresh: Instant = Instant::now();

    //Init various apps
    let mut cli = Cli::new();
    cli.set_state(true, true);
    let mut text_edit = TextEdit::new();
    let mut sprite_editor = SpriteEditor::new();

    let mut apps: Vec<Box<dyn Process>> = Vec::new();
    apps.push(Box::new(cli));
    apps.push(Box::new(text_edit));
    apps.push(Box::new(sprite_editor));

    let mut currently_running_app_index: usize = 0;

    let mut sprite0: Sprite = Sprite::new_from_file(&String::from("./resources/sprites/sprite1.txt"));
    sprite0.pos_x = VIRTUAL_WIDTH / 2;
    sprite0.pos_y = VIRTUAL_HEIGHT / 2;
    // let mut sprite1: Sprite = Sprite::new_from_file(&String::from("./resources/sprites/sprite1.txt"));
    // sprite1.pos_x = 30;
    // sprite1.pos_y = 30;
    // let mut sprite2: Sprite = Sprite::new_from_file(&String::from("./resources/sprites/sprite1.txt"));
    // sprite2.pos_x = 80;
    // sprite2.pos_y = 10;
    // let mut sprite3: Sprite = Sprite::new_from_file(&String::from("./resources/sprites/sprite1.txt"));
    // sprite3.pos_x = 150;
    // sprite3.pos_y = 80;
    // let mut sprite4: Sprite = Sprite::new_from_file(&String::from("./resources/sprites/sprite1.txt"));
    // sprite4.pos_x = 20;
    // sprite4.pos_y = 200;
    virtual_frame_buffer.get_sprite_list().push(sprite0);
    // virtual_frame_buffer.get_sprite_list().push(sprite1);
    // virtual_frame_buffer.get_sprite_list().push(sprite2);
    // virtual_frame_buffer.get_sprite_list().push(sprite3);
    // virtual_frame_buffer.get_sprite_list().push(sprite4);


    let mut key_released: Option<VirtualKeyCode> = None;
    let mut key_pressed_os: Option<VirtualKeyCode> = None;
    let mut char_received: Option<char> = None;
    let mut mouse_move_delta: (f64, f64) = (0.0, 0.0);

    event_loop.run(move |event, _, control_flow| {

        //Control_flow::Poll uses 100% of a CPU core (In Windows 10 at least). Use WaitUntil instead.
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

            if input.key_released(VirtualKeyCode::F1) {
                if currently_running_app_index == 0 {currently_running_app_index = 0} else {currently_running_app_index -= 1}

                for app in apps.chunks_exact_mut(1) {
                    app[0].set_state(false, false);
                }

                apps[currently_running_app_index].set_state(true, true);
            }

            if input.key_released(VirtualKeyCode::F2) {
                currently_running_app_index += 1;
                if currently_running_app_index == apps.len() {currently_running_app_index = apps.len() - 1}

                for app in apps.chunks_exact_mut(1) {
                    app[0].set_state(false, false);
                }

                apps[currently_running_app_index].set_state(true, true);
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
                // Application update code.
                let mut flow = None;
                
                for app in apps.chunks_exact_mut(1) {
                    if app[0].get_state().0 == true {
                        flow = app[0].update(char_received, key_pressed_os, key_released);
                    }
                }
                
                match flow {
                    Some(flow) => {
                        *control_flow = flow;
                    }

                    None => ()
                }

                for app in apps.chunks_exact_mut(1) {
                    if app[0].get_state().1 == true {
                        app[0].draw(&mut virtual_frame_buffer);
                    }
                }
                
                for sprite in virtual_frame_buffer.get_sprite_list() {
                    if mouse_move_delta.0 >= 0.0 {
                        sprite.pos_x += mouse_move_delta.0 as usize;
                    } else {
                        sprite.pos_x -= -mouse_move_delta.0 as usize;
                    }
                    
                    if mouse_move_delta.1 >= 0.0 {
                        sprite.pos_y += mouse_move_delta.1 as usize;
                    } else {
                        sprite.pos_y -= -mouse_move_delta.1 as usize;
                    }
                }

                char_received = None;
                key_pressed_os = None;
                key_released = None;
                mouse_move_delta.0 = 0.0;
                mouse_move_delta.1 = 0.0;

                virtual_frame_buffer.render();
                //draw_loading_border(virtual_frame_buffer.get_frame(), 20, 30);
                crt_renderer.render(&virtual_frame_buffer, pixels.get_frame());
                pixels.render().expect("Pixels render oups");
                window.request_redraw();
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
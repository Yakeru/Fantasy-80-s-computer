use crate::virtual_frame_buffer::{CrtEffectRenderer, VirtualFrameBuffer};
use app_macro::*;
use pixels::{Error, PixelsBuilder, SurfaceTexture};
use rand::Rng;
use std::time::Instant;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod config;
mod unicode;
mod characters_rom;
mod color_palettes;
mod sprite;
mod text_layer;
mod virtual_frame_buffer;

//Apps
mod apps;
use crate::apps::console::*;
use crate::apps::lines::*;
use crate::apps::shell::*;
use crate::apps::sprite_editor::*;
use crate::apps::squares::*;
use crate::apps::text_edit::*;
use crate::apps::weather_app::*;

//Settings
const FRAME_TIME_MS: u64 = 16; //ms per frame : 16 = 60fps, 32 = 30fps, 1000 = 1fps
const SPLASH: &str =
    " Fantasy CPC Microcomputer V(0.1)\u{000D}\u{000D} 2022 Damien Torreilles\u{000D}\u{000D}";

///*********************************************************THE MAIN
fn main() -> Result<(), Error> {
    //Custom intermediate frame buffer
    //Has 1/3 the horizontal resolution and 1/3 the vertical resoluton of pixels surface texture and winit window size.
    //The virtual frame buffer has a text layer, sprite lists, background layers and tiles layers that can be accessed
    //by Processes (structs implemeting "process") to build their image.
    //Its rendere combines all the layers in its frame to produce the complete image.
    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(FRAME_TIME_MS);

    //winit init and setup
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(
            virtual_frame_buffer.get_window_size().0 as i32,
            virtual_frame_buffer.get_window_size().1 as i32,
        ))
        .with_title("Yay, une fenêtre !")
        .with_resizable(false);
    //.with_fullscreen(Some(Fullscreen::Borderless(None)));
    let window = builder
        .build(&event_loop)
        .expect("Window creation failed !");

    window
        .set_cursor_grab(winit::window::CursorGrabMode::None)
        .unwrap();
    window.set_cursor_visible(false);

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(
            window_size.width,
            window_size.height,
            surface_texture,
        )
        .enable_vsync(false)
        .build()
        .expect("Pixels : Failed to setup rendering")
    };

    //The crt renderer takes the virtual frame buffers's frame, upscales it 3 times in X and Y to matche the pixcel's frame and winow size,
    //then applyes an effect to evoke CRT sub-pixels and scanlines.
    //The upscaled and "crt'ed" image is then pushed into pixel's frame for final render.
    let crt_renderer: CrtEffectRenderer = CrtEffectRenderer::new();

    //Init Shell
    //The Shell is the command line interpreter.
    //It is launched at startup, the winit event loop will update and render the shell by default if
    //no other process is running or has the focus.
    //It manages the start, stop, render, update of all the other processes.
    //It is always updated in the event loop event if another process has the focus (updated and rendered)

    // let mut shell = Box::new(Shell::new());
    // shell.set_state(true, true);
    // let mut text_edit = Box::new(TextEdit::new());
    // text_edit.set_state(false, false);
    // let mut sprite_edit = Box::new(SpriteEditor::new());
    // sprite_edit.set_state(false, false);
    let mut lines = Box::new(Lines::new());
    lines.set_state(true, true);
    let mut squares = Box::new(Squares::new());
    squares.set_state(false, false);
    // let mut weather_app = Box::new(WeatherApp::new());
    // weather_app.set_state(false, false);
    let mut console = Box::new(Console::new((5,5), (20,10)));
    console.set_state(true, true);

    //let mut app_list: Box<dyn AppMacro> = Vec::new();

    // let mut mouse_sprite: Sprite = Sprite::new_from_file(String::from("mouse"), &String::from("./resources/sprites/sprite1.txt"));
    // mouse_sprite.pos_x = VIRTUAL_WIDTH / 2;
    // mouse_sprite.pos_y = VIRTUAL_HEIGHT / 2;
    // virtual_frame_buffer.get_sprites().push(mouse_sprite);
    let mut mouse_move_delta: (f64, f64) = (0.0, 0.0);

    //Push the splash screen to the text layer
    virtual_frame_buffer.clear_frame_buffer(0);
    // virtual_frame_buffer
    //     .get_text_layer()
    //     .push_string(SPLASH, None, None, false);

    let mut keyboard_input: Option<KeyboardInput> = None;
    let mut char_received: Option<char> = None;

    let mut now = Instant::now();

    //The event loop here can be considered as a bios rom + terminal
    //it gathers all the keyborad inputs and sends them to the shell, the shell interprets them.
    //it always runs the shell and gives it the focus if no other app is running.
    event_loop.run(move |event, _, control_flow| {
        //Control_flow::Poll used 100% of one CPU core (In Windows 10 at least)
        //WaitUntil polls every "const FPS" ms instead: droped global CPU usage from 20% to 4%.
        //The whole program loops (updates and draws) at "const FPS" fps.
        //let refresh_timer: Instant = Instant::now().checked_add(Duration::from_millis(FRAME_TIME_MS)).unwrap();
        *control_flow = ControlFlow::Poll; //WaitUntil(refresh_timer);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    char_received = None;
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::ReceivedCharacter(c) => {
                    char_received = Some(c);
                    println!("Char received: {:?}", char_received);
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
                    ElementState::Pressed => (),
                    ElementState::Released => (),
                },
                DeviceEvent::Key(k) => {
                    keyboard_input = Some(k);
                    let toto = k.scancode;
                    let titi = k.state;
                    let tutu = k.virtual_keycode.unwrap();

                    println!(
                        "Scan: {}, state: {:?}, virt. key code: {:?}",
                        toto, titi, tutu
                    );
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                //Updating apps
                let process_response = console.update(keyboard_input, char_received);
                //let process_response = shell.update(keyboard_input, char_received);
                let process_response = lines.update(keyboard_input, char_received);
                // let process_response = squares.update(keyboard_input, char_received);
                //let process_response = text_edit.update(keyboard_input, char_received);
                //let process_response = sprite_edit.update(keyboard_input, char_received);

                //Process app response
                match process_response.event {
                    Some(event) => *control_flow = event,
                    None => (),
                }

                match process_response.message {
                    Some(message) => {
                        // virtual_frame_buffer
                        //     .get_text_layer()
                        //     .push_char('\u{000D}', None, None, false);
                        // virtual_frame_buffer
                        //     .get_text_layer()
                        //     .push_string(&message, None, None, false);
                    }
                    None => (),
                }

                //Draw app
                console.draw(&mut virtual_frame_buffer);
                //shell.draw(&mut virtual_frame_buffer);
                lines.draw(&mut virtual_frame_buffer);
                // squares.draw(&mut virtual_frame_buffer);
                //text_edit.draw(&mut virtual_frame_buffer);
                //sprite_edit.draw(&mut virtual_frame_buffer);
                //draw_loading_border(&mut virtual_frame_buffer, 40, 40); 

                //Render to frame buffer
                if now.elapsed().as_micros() >= (FRAME_TIME_MS * 1000) as u128 {
                    now = Instant::now();
                    //let render_time = Instant::now();
                    virtual_frame_buffer.render();
                    crt_renderer.render(&virtual_frame_buffer, pixels.get_frame(), true);
                    pixels.render().expect("Pixels render oups");
                    //println!("drawing: {} micros", render_time.elapsed().as_micros());
                }

                window.request_redraw();

                //Reset input buffers for next loop
                char_received = None;
                keyboard_input = None;
                mouse_move_delta.0 = 0.0;
                mouse_move_delta.1 = 0.0;
            }
            _ => (),
        }
    });
}

///Just for fun
fn draw_loading_border(
    virtual_frame_buffer: &mut VirtualFrameBuffer,
    vert_size: usize,
    horiz_size: usize,
) {
    let mut random = rand::thread_rng();
    let mut rgb_color: u8 = random.gen_range(0..32);

    let mut line_pixel_count: usize = 0;
    let mut line_count: usize = 0;
    let mut band_count: u8 = 0;
    let mut band: u8 = random.gen_range(0..20) + 4;

    let width = virtual_frame_buffer.get_width();
    let height = virtual_frame_buffer.get_height();

    for pixel in virtual_frame_buffer.get_frame().chunks_exact_mut(1) {
        if line_pixel_count < horiz_size
            || line_pixel_count > width - horiz_size
            || line_count < vert_size
            || line_count > height - vert_size
        {
            if band_count >= band {
                rgb_color = random.gen_range(0..32);
                band_count = 0;
                band = random.gen_range(0..20) + 4;
            }

            pixel[0] = rgb_color;
        }

        line_pixel_count += 1;

        if line_pixel_count == width {
            band_count += 1;
            line_count += 1;
            line_pixel_count = 0;
        }
    }
}

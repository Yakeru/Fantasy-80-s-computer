use virtual_frame_buffer::{*, color_palettes::{BLACK, WHITE}, text_layer_char::TextLayerChar, crt_renderer::CrtEffectRenderer, config::{VIRTUAL_WIDTH, VIRTUAL_HEIGHT}};
use app_macro::*;
use pixels::{Error, PixelsBuilder, SurfaceTexture};
use rand::Rng;
use std::time::Instant;
use winit::{
    dpi::{PhysicalSize, Position, PhysicalPosition},
    event::{DeviceEvent, ElementState, Event, KeyboardInput, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Fullscreen}
};

use unicode;

//Apps
mod apps;
use crate::apps::lines::*;
use crate::apps::shell::*;
use crate::apps::sprite_editor::*;
use crate::apps::squares::*;
use crate::apps::text_edit::*;
use crate::apps::weather_app::*;
use crate::apps::life::Life;

//Settings
const FRAME_TIME_MS: u128 = 16; //ms per frame : 16 = 60fps, 32 = 30fps, 1000 = 1fps
const FRAMES_PER_SEC: u128 = 60;

fn main() -> Result<(), Error> {

    // ************************************************ DISPLAY SETUP *********************************************
    // winit setup
    // For best effect, should display in border-less full-screen and native resolution on high DPI screen
    // This project was conceived with a recycled QHD iPAD pannel in mind
    // But winit can be set-up anyway you want.
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(
            config::WIDTH as i32,
            config::HEIGHT as i32,
        ))
        .with_title("Fantasy CPC")
        .with_resizable(false)
        .with_position(Position::Physical(PhysicalPosition::new(5, 5)));
    let window = builder
        .build(&event_loop)
        .expect("Window creation failed !");

    window
        .set_cursor_grab(winit::window::CursorGrabMode::None)
        .unwrap();

    for monitor in window.available_monitors() {
        if monitor.name().is_some() {
            if monitor.name().unwrap().contains("DISPLAY2") {
                window.set_decorations(false);
                window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
                break;
            }
        }
    }

    window.set_cursor_visible(true);

    // pixels set-up 
    // with the same goal in mind as winit's setup above, it is set to the exact same resolution as the window's 
    // inner size to avoid any scaling.
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(config::WIDTH as u32, config::HEIGHT as u32, &window);
        PixelsBuilder::new(
            config::WIDTH as u32,
            config::HEIGHT as u32,
            surface_texture,
        )
        .enable_vsync(false)
        .build()
        .expect("Pixels : Failed to setup rendering")
    };

    // **************************************************** GRAPHICS ENGINE SETUP **********************************************

    // Boolean used to play boot animation once.
    let mut booting = false;

    // The variables passed to the app.update(...) that is in focus
    // or to the shell if no other app is running.
    let mut keyboard_input: Option<KeyboardInput> = None;
    let mut char_received: Option<char> = None;
    let mut mouse_move_delta: (f64, f64) = (0.0, 0.0);
    let mut frame_counter:  u128 = 0;

    // Instant used to time the frame refresh rate
    // Apps are updated and drawn as frequently as possible, independently from that frame_interval
    // but the graphics engine (virtual_frame_buffer + crt_renderer + pixels) renders
    // the final on screen picture at this frame interval.
    let mut frame_interval = Instant::now();

    // My graphics engine
    // Offers a text layer, console, sprite layer, background layers and tiles layers that can be accessed
    // by Processes (structs implemeting "process") to build their image.
    // Its render combines all the layers in its frame, applies the crt filter and sends it to
    // pixels to display the final image in the window.
    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(FRAME_TIME_MS);

    // The crt renderer takes the virtual frame buffers's frame, upscales it to match pixel's frame and winit window size,
    // then applies a filter evoking CRT sub-pixels and scanlines.
    // The upscaled and "crt'ed" image is then pushed into pixel's frame for on-screen render.
    let mut crt_renderer: CrtEffectRenderer = CrtEffectRenderer::new(config::UPSCALE, true, u8::MAX);

    // ****************************************************** APPS SETUP ***********************************************
    
    // The Shell is the command line interpreter app.
    // It is launched at startup after the boot animation. 
    // The winit event loop will update and render the shell by default if
    // no other process is running or has the focus.
    // The Shell uses the console as default output.
    // When closing/quitting an app, it should always fall back to the shell.
    let mut shell = Box::new(Shell::new());
    shell.set_state(true, true);

    // To be managed properly, apps must be added to that list.
    // The main goes through the list and updates/renders the apps according to their statuses.
    let mut app_list: Vec<Box<dyn AppMacro>> = Vec::new();

    // ********* //
    // The apps  //
    // ********* //

    // LINES DEMO
    let lines = Box::new(Lines::new());
    app_list.push(lines);

    // SQUARES DEMO
    let squares = Box::new(Squares::new());
    app_list.push(squares);

    // TEXT EDITOR
    let text_edit = Box::new(TextEdit::new());
    app_list.push(text_edit);

    // SPRITE EDITOR
    let sprite_edit = Box::new(SpriteEditor::new());
    app_list.push(sprite_edit);

    // WEATHER APP
    let weather_app = Box::new(WeatherApp::new());
    app_list.push(weather_app);

    // CONWAY'S GAME OF LIFE, TEXT MODE
    let life = Box::new(Life::new());
    app_list.push(life);
    
    // ****************************************************** MAIN WINIT EVENT LOOP ***********************************************
    
    //The event loop here can be seen as the "bios + boot rom + console" part of the Fantasy computer.
    //It initialises the virtual_frame_buffer, Console 0 and Shell.
    //If no app is running/rendering, it defaults back to running/rendering the Console 0 and Shell.
    //It goes through app_list and updates all apps that have their update flag to true.
    //It goes through app_list and renders the appa that have their render flag and focus flag to true. Should be just one, so it stops at the first one it finds.
    //It reads the messages returned by the apps and displays them to Console 0.
    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    char_received = None;
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::ReceivedCharacter(c) => {
                    char_received = Some(c);
                    // println!("Char received: {:?}", char_received);
                }
                _ => {
                    char_received = None;
                }
            },
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    mouse_move_delta = delta;
                }
                DeviceEvent::Button { button, state } => {
                    match state {
                        ElementState::Pressed => (),
                        ElementState::Released => (),
                    };

                    match button {
                        0 => (),
                        1 => (),
                        _ => ()
                    }
                },
                DeviceEvent::Key(k) => {
                    keyboard_input = Some(k);
                    let scan_code = k.scancode;
                    let state = k.state;
                    let key_code = k.virtual_keycode.unwrap_or(VirtualKeyCode::NoConvert);

                    println!(
                        "Scan: {}, state: {:?}, virt. key code: {:?}",
                        scan_code, state, key_code
                    );
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                // BOOT, play boot animation once before showing the shell or any other app.
                if booting {
                    booting = boot_animation(&mut virtual_frame_buffer, &mut crt_renderer, frame_counter);
                } else {
                    //Updating apps
                    let mut show_shell: bool = true;
                    let mut app_response: Option<AppResponse> = None;
                    for app in app_list.chunks_exact_mut(1) {
                        
                        // If app is running and drawing (in focus), call update with keyboard inputs and dont render shell.
                        if app[0].get_state().0 && app[0].get_state().1 {
                            app_response = app[0].update(keyboard_input, char_received, &mut virtual_frame_buffer);
                            app[0].draw(&mut virtual_frame_buffer);
                            show_shell = false;
                        }
                        
                        // If app is running but not drawing (running in the background), call update without keyboard inputs.
                        // dont draw.
                        else if app[0].get_state().0 && !app[0].get_state().1 {
                            app_response = app[0].update(None, None, &mut virtual_frame_buffer);
                        }
                    }

                    // If no app is in focus, run the shell
                    if show_shell {
                        app_response = shell.update(keyboard_input, char_received, &mut virtual_frame_buffer);
                        shell.draw(&mut virtual_frame_buffer);
                    }

                    // Process app response
                    match app_response {
                        Some(response) => {
                            match response.event {
                                Some(event) => *control_flow = event,
                                None => (),
                            }

                            match response.message {
                                Some(message) => {
                                    println!("App message: {}", message);

                                    for app in app_list.chunks_exact_mut(1) {
                                        if app[0].get_name() == message {
                                            app[0].set_state(true, true);
                                        }
                                    }
                                }
                                None => (),
                            }
                        },
                        None => ()
                    }
                }

                // Render virtual frame buffer to pixels frame buffer with upscaling and CRT effect
                if frame_interval.elapsed().as_micros() >= FRAME_TIME_MS * 1000 {
                    frame_interval = Instant::now();
                    virtual_frame_buffer.render();

                    // let start = Instant::now();
                    crt_renderer.render(&mut virtual_frame_buffer, pixels.get_frame_mut());
                    // println!("Render time: {} micros", start.elapsed().as_micros());
                    pixels.render().expect("Pixels render oups");
                    frame_counter = frame_counter + 1;
                }

                window.request_redraw();

                // Reset input buffers for next loop
                char_received = None;
                keyboard_input = None;
                mouse_move_delta.0 = 0.0;
                mouse_move_delta.1 = 0.0;
            }
            _ => (),
        }
    });
}

///Just for fun, random colored lines in overscan zone, Amstrad style
fn draw_loading_border(virtual_frame_buffer: &mut VirtualFrameBuffer) {
    let mut random = rand::thread_rng();
    let mut rgb_color: u8 = random.gen_range(0..32);

    let mut line_pixel_count: usize = 0;
    let mut line_count: usize = 0;
    let mut band_count: u8 = 0;
    let mut band: u8 = random.gen_range(0..20) + 4;

    let width = virtual_frame_buffer.get_width();
    let height = virtual_frame_buffer.get_height();
    let horiz_size = (virtual_frame_buffer.get_width() - virtual_frame_buffer.get_text_layer_size_xy().0 * 8)/2;
    let vert_size = (virtual_frame_buffer.get_height() - virtual_frame_buffer.get_text_layer_size_xy().1 * 8)/2;

    for pixel in virtual_frame_buffer.get_frame_mut().chunks_exact_mut(1) {
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

///Boot animation
fn boot_animation(virtual_frame_buffer: &mut VirtualFrameBuffer, crt_renderer: &mut CrtEffectRenderer, frame_counter: u128) -> bool {
    
    virtual_frame_buffer.get_console_mut().display = false;

    //CRT warm up
    let br = if frame_counter > 255 {255} else {frame_counter as u8};
    crt_renderer.set_brightness(br);

    //Fill text layer with random garbage
    if frame_counter == 0 {
        genrate_random_garbage(virtual_frame_buffer);
    }

    //Clear garbage and display char and color test after 2 seconds
    if frame_counter == FRAMES_PER_SEC * 3 {

        //Clear text layer
        virtual_frame_buffer.get_text_layer_mut().clear();

        //Clear frame buffer
        virtual_frame_buffer.clear_frame_buffer(0);

        //Display all possible colors on first row
        // for i in 0..32_u8 {
        //     virtual_frame_buffer.get_text_layer_mut().insert_char(i as usize, ' ', Some(BLACK), Some(i), false, false, false);
        // }

        //Display all chars starting on second row
        // let width = virtual_frame_buffer.get_text_layer_size_xy().0;
        // for i in 0..characters_rom::ROM.len() {
        //     virtual_frame_buffer.get_text_layer_mut().insert_char(width + i as usize, characters_rom::CHARS[i], Some(WHITE), Some(BLACK), false, false, false);
        // }

        virtual_frame_buffer.get_text_layer_mut().insert_string_xy(0, 0, "Loading..." , Some(WHITE), Some(BLACK), false, false, false);
    }

    //Display loading overscan while "loading"
    if frame_counter >= FRAMES_PER_SEC * 3 && frame_counter <= FRAMES_PER_SEC * 6 {
        draw_loading_border(virtual_frame_buffer);
    }
    
    if frame_counter >= 6 * FRAMES_PER_SEC {
        virtual_frame_buffer.get_text_layer_mut().clear();
        virtual_frame_buffer.clear_frame_buffer(0);
        return false;
    }
    else {
        return true;
    } 
}

pub fn genrate_random_garbage(virtual_frame_buffer: &mut VirtualFrameBuffer) {

    let mut random = rand::thread_rng();
        
    let frame: u8 = random.gen_range(0..32);
    virtual_frame_buffer.clear_frame_buffer(frame);
    virtual_frame_buffer.get_text_layer_mut().clear();

    let char_map = virtual_frame_buffer.get_text_layer_mut().get_char_map_mut();
    for index in 0..char_map.len() {
        
        let mut color: u8 = random.gen_range(0..40);
        color = if color > 31 { 0 } else { color };

        let mut bkg_color: u8 = random.gen_range(0..40);
        bkg_color = if bkg_color > 31 { 0 } else { bkg_color };
        
        let mut char_index = random.gen_range(0..100);
        char_index = if char_index > characters_rom::CHARS.len() - 1 { 0 } else { char_index };
        let c:char = characters_rom::CHARS[char_index];

        let effect:u8 = random.gen_range(0..10);
        let swap: bool = if effect & 0b00000001 > 0 {true} else {false};
        let blink: bool = if effect & 0b00000010 > 0 {true} else {false};
        let shadowed: bool = if effect & 0b00000100 > 0 {true} else {false};

        let text_layer_char: TextLayerChar = TextLayerChar{c, color, bkg_color, swap, blink, shadowed};
        char_map[index] = Some(text_layer_char);
    }
}
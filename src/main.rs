use rodio::{OutputStream, Sink, Source};
use crt_shader_renderer::CrtRenderer;
use sound::{notes::*, play};
use display_controller::{*, color_palettes::{BLACK, WHITE}, text_layer::TextLayerChar, renderer::Renderer, config::{HEIGHT, WIDTH, VIRTUAL_HEIGHT, VIRTUAL_WIDTH, FULLSCREEN, UPSCALE}};
use app_macro::*;
use pixels::{Error, PixelsBuilder, SurfaceTexture};
use rand::Rng;
use winit_input_helper::WinitInputHelper;
use std::{time::{Duration, Instant}, thread};
use winit::{
    dpi::{PhysicalSize, Position, PhysicalPosition},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Fullscreen}, event::Event
};

use clock::Clock;

mod crt_shader_renderer;

//Apps
mod apps;
use crate::apps::shell::*;
use crate::apps::life::*;
use crate::apps::weather_app::*;
use crate::apps::mandelbrot::*;

//Sound
mod sound;
use crate::play::play;

//Settings
//const FRAME_TIME_MS: u128 = 16; //ms per frame : 16 = 60fps, 32 = 30fps, 1000 = 1fps
//const FRAMES_PER_SEC: u128 = 60;

fn main() -> Result<(), Error> {

    // ************************************************* SOUND TEST **********************************************    

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let channel_1 = Sink::try_new(&stream_handle).unwrap();
    let channel_2 = Sink::try_new(&stream_handle).unwrap();
    //let channel_3 = Sink::try_new(&stream_handle).unwrap();
    //let channel_4 = Sink::try_new(&stream_handle).unwrap();

    // let _handle = thread::Builder::new().name("sound".to_string()).spawn(move || {

    //     let mut melody_1: Vec<Option<(f32, f32)>> = Vec::new();
    //     melody_1.push(Some((0.0, 10.0)));
    //     melody_1.push(Some((C5, 1.0)));
    //     melody_1.push(None);
    //     melody_1.push(Some((C5, 1.0)));
    //     melody_1.push(Some((F5, 2.0)));

    //     let mut melody_2: Vec<Option<(f32, f32)>> = Vec::new();
    //     melody_2.push(Some((0.0, 10.0)));
    //     melody_2.push(Some((0.0, 3.0)));
    //     melody_2.push(Some((A5, 2.0)));

    //     play(480.0, &melody_1, &melody_2, &channel_1, &channel_2);
    // });
    
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

    if FULLSCREEN { 
        window.set_decorations(false);
        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    }

    for monitor in window.available_monitors() {
        if monitor.name().is_some() {
            if monitor.name().unwrap().contains("DISPLAY2") {
                if FULLSCREEN { 
                    window.set_decorations(false);
                    window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
                }
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
            config::VIRTUAL_WIDTH as u32,
            config::VIRTUAL_HEIGHT as u32,
            surface_texture,
        )
        .enable_vsync(true)
        .build()
        .expect("Pixels : Failed to setup rendering")
    };

    // **************************************************** GRAPHICS ENGINE SETUP **********************************************

    // The "system clock"
    let mut system_clock: Clock = Clock::new();

    // Boolean used to play boot animation once.
    let mut booting = true;

    // The variables passed to the app.update(...) that is in focus
    // or to the shell if no other app is running.
    let mut mouse_move_delta: (f64, f64) = (0.0, 0.0);

    // Fantasy CPC graphics engine
    // Offers a text layer, console, sprite layer, background layers and tiles layers that can be accessed
    // by Processes (structs implemeting "process") to build their image.
    // Its render combines all the layers in its frame, applies the crt filter and sends it to
    // pixels to display the final image in the window.
    let mut display_controller: DisplayController = DisplayController::new();

    // The software crt renderer takes the virtual frame buffers's frame, upscales it to match pixel's frame and winit window size,
    // then applies a filter evoking CRT sub-pixels and scanlines.
    // The upscaled and "crt'ed" image is then pushed into pixel's frame for on-screen render.
    let mut renderer: Renderer = Renderer::new(u8::MAX);

    // A crt renderer using pixels upscaler and a CRT shader in WGSL
    let mut display_mode: u32 = 0;
    let mut mask_type: u32 = 0;
    let mut distortion: u32 = 0;

    let crt_shader_renderer = CrtRenderer::new(&pixels, WIDTH as u32, HEIGHT as u32, display_mode, UPSCALE as u32, (UPSCALE/2) as u32, mask_type as u32, distortion as u32)?;

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

    // CONWAY'S GAME OF LIFE, TEXT MODE
    let life = Box::new(Life::new());
    app_list.push(life);

    // WEATHER APP
    let weather_app = Box::new(WeatherApp::new());
    app_list.push(weather_app);

    // MANDELBROT
    let mandelbrot = Box::new(Mandelbrot::new());
    app_list.push(mandelbrot);
    
    let mut frame_time_100: Vec<u128> = Vec::new();
    let mut time = 0.0;

    // ****************************************************** MAIN WINIT EVENT LOOP ***********************************************
    
    let mut input = WinitInputHelper::new();
    
    //The event loop here can be seen as the "bios + boot rom + console" part of the Fantasy computer.
    //It initialises the display_controller, Console 0 and Shell.
    //If no app is running/rendering, it defaults back to running/rendering the Console 0 and Shell.
    //It goes through app_list and updates all apps that have their update flag to true.
    //It goes through app_list and renders the appa that have their render flag and focus flag to true. Should be just one, so it stops at the first one it finds.
    //It reads the messages returned by the apps and displays them to Console 0.
    event_loop.run(move |event, _, control_flow| {

        // let now = Instant::now();
        // let plop = Duration::from_millis(2);
        //*control_flow = ControlFlow::WaitUntil(now.checked_add(plop).unwrap());
        *control_flow = ControlFlow::Poll; //Poll is synchronized with V-Sync

        if let Event::RedrawRequested(_) = event {
                    
            let render_result = pixels.render_with(|encoder, render_target, context| {
                let noise_texture = crt_shader_renderer.texture_view();
                context.scaling_renderer.render(encoder, noise_texture);

                crt_shader_renderer.update(&context.queue, WIDTH as f32, HEIGHT as f32, display_mode as f32, UPSCALE as f32, (UPSCALE/2) as f32, mask_type as f32, distortion as f32);

                crt_shader_renderer.render(encoder, render_target, context.scaling_renderer.clip_rect());

                Ok(())
            });

            if let Err(err) = render_result {
                //log_error("pixels.render_with", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {

            system_clock.update();

            //println!("second tick: {}, half second tick: {}, frames: {}", system_clock.second_tick, system_clock.half_second_tick, system_clock.get_frame_count());

            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit
            }

            // BOOT, play boot animation once before showing the shell or any other app.
            if booting {
                booting = boot_animation(&mut display_controller, &mut renderer, &system_clock);
            } else {
                //Updating apps
                let mut show_shell: bool = true;
                let mut app_response: Option<AppResponse> = None;
                //let app_inputs: AppInputs = AppInputs { keyboard_input, char_received, mouse_move_delta, system_clock };
                for app in app_list.chunks_exact_mut(1) {
                    
                    // If app is running and drawing (in focus), call update with keyboard inputs and dont render shell.
                    if app[0].get_state().0 && app[0].get_state().1 {
                        app_response = app[0].update(&input, &system_clock, &mut display_controller);
                        app[0].draw(&input, &system_clock, &mut display_controller);
                        show_shell = false;
                    }
                    
                    // If app is running but not drawing (running in the background), call update without keyboard inputs.
                    // dont draw.
                    else if app[0].get_state().0 && !app[0].get_state().1 {
                        app_response = app[0].update(&input, &system_clock, &mut display_controller);
                    }
                }

                // If no app is in focus, run the shell
                if show_shell {
                    app_response = shell.update(&input, &system_clock, &mut display_controller);
                    shell.draw(&input, &system_clock, &mut display_controller);
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
                                };

                                if message == String::from("mode 0") {
                                    display_mode = 0;
                                }

                                if message == String::from("mode 1") {
                                    display_mode = 1;
                                }

                                if message == String::from("mode 2") {
                                    display_mode = 2;
                                }

                                if message == String::from("dist 0") {
                                    distortion = 0;
                                }

                                if message == String::from("dist 1") {
                                    distortion = 42;
                                }

                                if message == String::from("dist 2") {
                                    distortion = 16;
                                }

                                if message == String::from("dist 3") {
                                    distortion = 2;
                                }
                            }
                            
                            None => (),
                        }
                    },
                    None => ()
                }
            }

            // Render virtual frame buffer to pixels frame buffer with upscaling and CRT effect
            let start = Instant::now();

            display_controller.render();
        
            renderer.render(&mut display_controller.get_frame(), pixels.frame_mut());
            
            frame_time_100.push(start.elapsed().as_micros());
            
            if frame_time_100.len() == 100 {
                
                let mut total_time: u128 = 0;
                
                for time in &frame_time_100 {
                    total_time += time;
                }

                let avg = total_time/100;

                println!("Render time: {} micros", avg);
                frame_time_100.clear();
            }
            
            //pixels.render().expect("Pixels render oups");
            window.request_redraw();
            system_clock.count_frame();

            // Reset input buffers for next loop
            mouse_move_delta.0 = 0.0;
            mouse_move_delta.1 = 0.0;
        }
    });
}

///Just for fun, random colored lines in overscan zone, Amstrad style
fn draw_loading_border(display_controller: &mut DisplayController) {
    let mut random = rand::thread_rng();
    let mut rgb_color: u8 = random.gen_range(0..32);
    let mut line_count: usize = 0;
    let mut band_height: usize = random.gen_range(4..20);

    while line_count <= VIRTUAL_HEIGHT {
        let range_max = if line_count + band_height > VIRTUAL_HEIGHT {VIRTUAL_HEIGHT } else { line_count + band_height };
        display_controller.set_overscan_color_range(rgb_color, line_count..range_max);
        line_count += band_height;
        rgb_color = random.gen_range(0..32);
        band_height = random.gen_range(4..20);
    }
}

///Boot animation
fn boot_animation(display_controller: &mut DisplayController, crt_renderer: &mut Renderer, clock: &Clock) -> bool {
    
    display_controller.get_console_mut().display = false;

    //CRT warm up, brightness increases from 0 to 255 in 2 seconds
    let brigthness = if clock.total_running_time >= Duration::new(2, 0) {255} else {(clock.total_running_time.as_millis() * 255 / 2000) as u8};
    crt_renderer.set_brightness(brigthness);

    //Fill text layer with random garbage
    if clock.get_frame_count() == 0 {
        genrate_random_garbage(display_controller);
    }

    //Clear garbage and display Loading...
    if clock.total_running_time >= Duration::new(3, 0) {
        display_controller.get_text_layer_mut().clear();
        display_controller.clear(0);
        display_controller.get_text_layer_mut().insert_string_xy(0, 0, "Loading..." , Some(WHITE), Some(BLACK), false, false, false);
    }

    //Display loading overscan while "loading"
    if clock.total_running_time >= Duration::new(3, 0) && clock.total_running_time < Duration::new(6, 0) {
        draw_loading_border(display_controller);
    }
    
    if clock.total_running_time >= Duration::new(6, 0) {
        display_controller.get_text_layer_mut().clear();
        display_controller.clear(0);
        return false;
    }
    else {
        return true;
    } 
}

pub fn genrate_random_garbage(display_controller: &mut DisplayController) {

    let mut random = rand::thread_rng();
        
    let frame: u8 = random.gen_range(0..32);
    display_controller.clear(frame);
    display_controller.get_text_layer_mut().clear();

    let char_map = display_controller.get_text_layer_mut().get_char_map_mut();
    for index in 0..char_map.len() {
        
        let mut color: u8 = random.gen_range(0..40);
        color = if color > 31 { 0 } else { color };

        let mut bkg_color: u8 = random.gen_range(0..40);
        bkg_color = if bkg_color > 31 { 0 } else { bkg_color };
        
        let mut char_index = random.gen_range(0..100);
        char_index = if char_index > characters_rom::CHAR_TABLE.len() - 1 { 0 } else { char_index };
        let c:char = characters_rom::CHAR_TABLE[char_index];

        let effect:u8 = random.gen_range(0..10);
        let swap: bool = if effect & 0b00000001 > 0 {true} else {false};
        let blink: bool = if effect & 0b00000010 > 0 {true} else {false};
        let shadowed: bool = if effect & 0b00000100 > 0 {true} else {false};

        let text_layer_char: TextLayerChar = TextLayerChar{c, color, bkg_color, swap, blink, shadowed};
        char_map[index] = Some(text_layer_char);
    }
}
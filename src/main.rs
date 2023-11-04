use app_macro::*;
use apps::{
    boot::Boot, cli::shell::Shell, life::Life, mandelbrot::game::Mandelbrot,
    raycaster::game::Raycaster, weather_app::WeatherApp,
};
use crt_shader_renderer::CrtRenderer;
use display_controller::{config::*, *};
use pixels::{Error, PixelsBuilder, SurfaceTexture};
use rodio::Source;
use shader_variables::ShaderVariables;
use sound::play;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize, Position},
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use clock::Clock;

mod apps;
mod crt_shader_renderer;
mod shader_variables;
mod sound;

fn main() -> Result<(), Error> {
    // ************************************************ SOUND INIT ************************************************
    // First time sound is played, it takes a few seconds and gets de-sync'ed with the display
    // So here is a function to play an empty sound for 1/10 s to "init" rodio
    play::init_sound();

    // ************************************************ DISPLAY SETUP *********************************************
    // winit setup
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(
            display_controller::config::SCREEN_WIDTH as i32,
            display_controller::config::SCREEN_HEIGHT as i32,
        ))
        .with_title("Fantasy CPC")
        .with_resizable(false)
        .with_position(Position::Physical(PhysicalPosition::new(5, 5)));
    let window = window_builder
        .build(&event_loop)
        .expect("Window creation failed !");

    window
        .set_cursor_grab(winit::window::CursorGrabMode::None)
        .unwrap();

    for monitor in window.available_monitors() {
        if monitor.size().width >= SCREEN_WIDTH as u32
            && monitor.size().height >= SCREEN_HEIGHT as u32
        {
            if FULLSCREEN {
                window.set_decorations(false);
                window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
            }
            break;
        }
    }

    window.set_cursor_visible(true);

    // pixels set-up
    let mut pixels = {
        let surface_texture = SurfaceTexture::new(
            display_controller::config::SCREEN_WIDTH as u32,
            display_controller::config::SCREEN_HEIGHT as u32,
            &window,
        );
        PixelsBuilder::new(
            display_controller::config::VIRTUAL_WIDTH as u32,
            display_controller::config::VIRTUAL_HEIGHT as u32,
            surface_texture,
        )
        .enable_vsync(true)
        .build()
        .expect("Pixels : Failed to setup rendering")
    };

    // **************************************************** GRAPHICS ENGINE SETUP **********************************************

    // The "system clock"
    let mut system_clock: Clock = Clock::new();

    // The variables passed to the app.update(...) that is in focus
    // or to the shell if no other app is running.
    let mut mouse_move_delta: (f64, f64) = (0.0, 0.0);

    // Fantasy CPC graphics engine
    // Offers a text layer, console, sprite layer, background layers and tiles layers that can be accessed
    // by Processes (structs implemeting "process") to build their image.
    // Its render combines all the layers in its frame, applies the crt filter and sends it to
    // pixels to display the final image in the window.
    let mut display_controller: DisplayController = DisplayController::new();

    // A crt renderer using pixels upscaler and a CRT shader in WGSL
    let mut shader_variables: ShaderVariables = ShaderVariables::new();
    let crt_renderer = CrtRenderer::new(&pixels, &shader_variables)?;

    // ****************************************************** APPS SETUP ***********************************************

    // The Shell is the command line interpreter app.
    // It is launched at startup after the boot animation.
    // The winit event loop will update and render the shell by default if
    // no other process is running or has the focus.
    // The Shell uses the console as default output.
    // When closing/quitting an app, it should always fall back to the shell.
    let mut shell = Box::new(Shell::new());
    shell.set_state(AppStatus::Running);

    // ********* //
    // The apps  //
    // ********* //

    // To be managed properly, apps must be added to that list.
    // The main loop goes through the list and updates/renders the apps according to their statuses.
    let mut app_list: Vec<Box<dyn AppMacro>> = Vec::new();

    // BOOT APP, not really an app, just plays the animation at startup, and when "reboot" command is sent
    let boot = Box::new(Boot::new());
    app_list.push(boot);

    // CONWAY'S GAME OF LIFE, TEXT MODE
    let life = Box::new(Life::new());
    app_list.push(life);

    // WEATHER APP
    let weather_app = Box::new(WeatherApp::new());
    app_list.push(weather_app);

    // MANDELBROT
    let mandelbrot = Box::new(Mandelbrot::new());
    app_list.push(mandelbrot);

    // RAYCASTER
    let raycaster = Box::new(Raycaster::new());
    app_list.push(raycaster);

    // ****************************************************** MAIN WINIT EVENT LOOP ***********************************************

    let mut input = WinitInputHelper::new();

    //The event loop here can be seen as the "bios + boot rom + console" part of the Fantasy computer.
    //It initialises the display_controller, Console 0 and Shell.
    //If no app is running/rendering, it defaults back to running/rendering the Console 0 and Shell.
    //It goes through app_list and updates all apps that have their update flag to true.
    //It goes through app_list and renders the apps that have their render flag and focus flag to true. Should be just one, so it stops at the first one it finds.
    //It reads the messages returned by the apps displays them to Console 0 and interpets them.
    //TODO It sends messages to apps
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll; //Poll is synchronized with V-Sync

        if let Event::RedrawRequested(_) = event {
            let render_result = pixels.render_with(|encoder, render_target, context| {
                let texture = crt_renderer.texture_view();
                context.scaling_renderer.render(encoder, texture);
                crt_renderer.update(&context.queue, &shader_variables);
                crt_renderer.render(encoder, render_target, context.scaling_renderer.clip_rect());
                Ok(())
            });
            if let Err(err) = render_result {
                println!("Rendering error : {}", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            system_clock.update();

            // If user clicks on cross to close window for example
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit
            }

            //Updating apps
            let mut show_shell: bool = true;
            let mut app_response: Option<AppResponse> = None;
            //let app_inputs: AppInputs = AppInputs { keyboard_input, char_received, mouse_move_delta, system_clock };
            for app in app_list.chunks_exact_mut(1) {
                if *app[0].get_state() == AppStatus::Running {
                    show_shell = false;
                };

                app[0].exec_app(Some(&input), &system_clock, &mut display_controller);
            }

            // If no app is in focus, run the shell
            if show_shell {
                app_response = shell.exec_app(Some(&input), &system_clock, &mut display_controller);
            }

            // Process app response
            //TODO make smarter command line tokenizer and interpreter
            //TODO make enum of available commands
            //TODO add support for command line parameters
            if let Some(response) = app_response {
                //If app response contains a winit event, send it directly to winit's control flow
                if let Some(app_event) = response.event {
                    *control_flow = app_event
                }

                //If app_response contains a message, interpret it with command line interpreter.
                //ex: when pressing enter in console app, the message will contain the command from the console
                //that command can then be interpreted to run another app, or change settings.
                if let Some(app_message) = response.message {
                    println!("App message: {}", app_message);

                    //Tests if message is name of an available app. If so, switches to that app.
                    for app in app_list.chunks_exact_mut(1) {
                        if app[0].get_name() == app_message {
                            app[0].set_state(AppStatus::Running);
                        }
                    }

                    //Reboot (resets app statuses to default state and plays boot animation)
                    if app_message == "reboot" {
                        shell.set_state(AppStatus::Stopped);
                        shell.set_state(AppStatus::Running);
                    }

                    //Shader settings
                    if app_message == "mode 0" {
                        shader_variables.mode = 0.0;
                    }

                    if app_message == "mode 1" {
                        shader_variables.mode = 1.0;
                    }

                    if app_message == "mode 2" {
                        shader_variables.mode = 2.0;
                    }

                    if app_message == "dist 0" {
                        shader_variables.horiz_distortion = 0.0;
                        shader_variables.vert_distortion = 0.0;
                    }

                    if app_message == "dist 1" {
                        shader_variables.horiz_distortion = 32.0 * (4.0 / 3.0);
                        shader_variables.vert_distortion = 32.0;
                    }

                    if app_message == "dist 2" {
                        shader_variables.horiz_distortion = 16.0 * (4.0 / 3.0);
                        shader_variables.vert_distortion = 16.0;
                    }

                    if app_message == "dist 3" {
                        shader_variables.horiz_distortion = 8.0 * (4.0 / 3.0);
                        shader_variables.vert_distortion = 8.0;
                    }

                    if app_message == "dist 4" {
                        shader_variables.horiz_distortion = 2.0 * (4.0 / 3.0);
                        shader_variables.vert_distortion = 2.0;
                    }

                    if app_message == "dist 5" {
                        shader_variables.horiz_distortion = 1.0 * (4.0 / 3.0);
                        shader_variables.vert_distortion = 1.0;
                    }

                    if app_message == "dist 6" {
                        shader_variables.horiz_distortion = 0.5 * (4.0 / 3.0);
                        shader_variables.vert_distortion = 0.5;
                    }
                }
            }

            //Combine all the layers, render text, render sprites, etc...
            //into pixel's frame buffer
            display_controller.render(pixels.frame_mut());

            window.request_redraw();
            system_clock.count_frame();

            // Reset mouse delta for next loop/frame
            mouse_move_delta.0 = 0.0;
            mouse_move_delta.1 = 0.0;
        }
    });
}

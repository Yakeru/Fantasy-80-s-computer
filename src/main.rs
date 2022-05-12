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
    Instant, Duration
};
use std::io::{self, Write};

mod characters_rom;
mod text_layer;
mod virtual_frame_buffer;
mod color_palettes;
mod app;
mod cli;
mod text_edit;

use crate::text_layer::{TextLayer, TextLayerRenderer};
use crate::virtual_frame_buffer::{VirtualFrameBuffer, CrtEffectRenderer};
use crate::app::*;
use crate::cli::*;
use crate::text_edit::*;

const WIDTH: usize = 1280;
const HEIGHT: usize = 960;

const FPS: u64 = 16; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

const DEFAULT_BKG_COLOR: u8 = 4;
const TEXT_COLUMNS: usize = 40;
const TEXT_ROWS: usize = 25;

const VIRTUAL_WIDTH: usize = 426;  //426*3 = 1278 draw one black line on each side of screen for perfectly centered *3 scale
const VIRTUAL_HEIGHT: usize = 240; //240*4 = 960

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
    
    //TODO in the future : have a surface texture upscale the buffer instead of upscaling by hand pixel by pixel ...
    //and learn to do shaders for the CRT effect.
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    let mut text_layer = TextLayer::new(TEXT_COLUMNS, TEXT_ROWS);
    let text_renderer = TextLayerRenderer::new(TEXT_COLUMNS, TEXT_ROWS, VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, TEXT_COLUMNS, TEXT_ROWS);
    let crt_renderer: CrtEffectRenderer = CrtEffectRenderer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, WIDTH, HEIGHT);

    let mut last_refresh: Instant = Instant::now();

    //Init various apps
    let mut cli = Cli::new(1);
    let mut text_edit = TextEdit::new(2);
    text_edit.app.updating = false;
    text_edit.app.drawing = false;
    let mut running_apps: Vec<&App> = Vec::new();
    running_apps.push(&cli.app);
    running_apps.push(&text_edit.app);

    let mut key_released: Option<VirtualKeyCode> = None;
    let mut char_received: Option<char> = None;

    event_loop.run(move |event, _, control_flow| {

        //Control_flow::Poll uses 100% of a CPU core (In Windows 10 at least). Use WaitUntil instead.
        let refresh_timer: Instant = Instant::now().checked_add(Duration::from_millis(FPS)).unwrap();
        *control_flow = ControlFlow::WaitUntil(refresh_timer);

        if input.update(&event) {
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
                cli.app.updating = true;
                cli.app.drawing = true;
                text_edit.app.updating = false;
                text_edit.app.drawing = false;
            }

            if input.key_released(VirtualKeyCode::F2) {
                cli.app.updating = false;
                cli.app.drawing = false;
                text_edit.app.updating = true;
                text_edit.app.drawing = true;
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
            Event::MainEventsCleared => {
                // Application update code.
                let mut flow = None;
                
                if  cli.app.updating {
                    flow  = cli.update(char_received, key_released);
                }

                if  text_edit.app.updating {
                    flow  = text_edit.update(char_received, key_released);
                }
                
                match flow {
                    Some(flow) => {
                        *control_flow = flow;
                    }

                    None => ()
                }

                char_received = None;
                key_released = None;

                if  cli.app.drawing {
                    cli.draw_text(&mut text_layer);
                }

                if  text_edit.app.drawing {
                    text_edit.draw_text(&mut text_layer);
                }

                //let render_time: Instant = Instant::now();
                virtual_frame_buffer.clear_frame_buffer(DEFAULT_BKG_COLOR);
                draw_loading_border(virtual_frame_buffer.get_frame(), 20, 30);
                text_renderer.render(&text_layer, &mut virtual_frame_buffer);
                crt_renderer.render(virtual_frame_buffer.get_frame(), pixels.get_frame());
                //println!("draw time {}us", render_time.elapsed().as_micros());
                pixels.render().expect("Pixels render oups");
                window.request_redraw();
            }
            _ => ()
        }
    });
}

fn draw_loading_border(frame_buffer: &mut[u8], vert_size: usize, horiz_size: usize) {
    let mut random = rand::thread_rng();
    let mut rgb_color: u8 = random.gen_range(0..8);

    let mut line_pixel_count: usize = 0;
    let mut line_count: usize = 0;
    let mut band_count: u8 = 0;
    let mut band: u8 = random.gen_range(0..20) + 4;

    for pixel in frame_buffer.chunks_exact_mut(1) {

        if line_pixel_count < horiz_size || line_pixel_count > VIRTUAL_WIDTH - horiz_size || line_count < vert_size || line_count > VIRTUAL_HEIGHT - vert_size {
            if band_count >= band {
                rgb_color = random.gen_range(0..8);
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
use winit::{
    event::{Event, WindowEvent, VirtualKeyCode, KeyboardInput},
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
mod virtual_text_mode;
mod virtual_frame_buffer;
mod color_palettes;
mod boot_rom;
mod shell;

use crate::virtual_text_mode::{VirtualTextLayerFrameBuffer, TextLayerRenderer};
use crate::virtual_frame_buffer::{VirtualFrameBuffer, CrtEffectRenderer};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;

const VIRTUAL_WIDTH: u32 = 426;  //426*3 = 1278 draw one black line on each side of screen for perfect *3 scale
const VIRTUAL_HEIGHT: u32 = 240; //240*4 = 960

const FPS: u128 = 16; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

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
    
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut text_color = DEFAULT_COLOR;
    let mut text_bkg_color = DEFAULT_BKG_COLOR;
    let mut virtual_text_layer_buffer = VirtualTextLayerFrameBuffer::new(TEXT_COLUMNS as u32, TEXT_ROWS as u32);
    let text_renderer = TextLayerRenderer::new(TEXT_COLUMNS as u32, TEXT_ROWS as u32, VIRTUAL_WIDTH, VIRTUAL_HEIGHT);

    let mut virtual_frame_buffer: VirtualFrameBuffer = VirtualFrameBuffer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
    let crt_renderer: CrtEffectRenderer = CrtEffectRenderer::new(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, WIDTH, HEIGHT);

    //let mut shell: Shell = Shell::new(5000, TEXT_COLUMNS, TEXT_ROWS);

    let mut last_refresh: Instant = Instant::now();

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
            }

            if input.key_released(VirtualKeyCode::Right) {
                if text_color == 0 {text_color = 7} else {text_color -= 1}
            }

            if input.key_released(VirtualKeyCode::Up) {
                if text_bkg_color == 7 {text_bkg_color = 0} else {text_bkg_color += 1}
            }

            if input.key_released(VirtualKeyCode::Down) {
                if text_bkg_color == 0 {text_bkg_color = 7} else {text_bkg_color -= 1}
            }
        }

        match event {

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::ReceivedCharacter(c) => {
                    
                    print!("{}", c);
                    io::stdout().flush().unwrap();

                    if c == 0x08 as char { //0x08 is unicode for Backspace
                        virtual_text_layer_buffer.pop_char();
                    } else {
                        virtual_text_layer_buffer.push_char(c, text_color, text_bkg_color, false);
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
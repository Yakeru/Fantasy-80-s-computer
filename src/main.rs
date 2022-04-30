use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::PhysicalSize,
};

use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use std::time::{
    Instant
};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 960;
const FPS: u128 = 16; //ms per frame, so 16 = 60fps, 32 = 30fps, 1000 = 1fps

const BORDER: u32 = 100;
const BKG_COLOR: (u8, u8, u8) = (0, 0, 254);

fn main()-> Result<(), Error> {

    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_title("Yay, une fenêtre !")
        .with_resizable(false);

    let window = builder.build(&event_loop).expect("Window creation failed !");
    
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut last_refresh: Instant = Instant::now();

    let mut sprite: Sprite = Sprite::new();
    sprite.position.x = 150;
    sprite.position.y = 150;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                // Application update code.
    
                // Queue a RedrawRequested event.
                if last_refresh.elapsed().as_millis() >= FPS {
                    let render_time: Instant = Instant::now();
                    draw_loading_border(pixels.get_frame());
                    draw_background(pixels.get_frame());
                    apply_crt_effect(pixels.get_frame());
                    println!("draw time {}us", render_time.elapsed().as_micros());
                    pixels.render().expect("Pixels render oups");
                    window.request_redraw();
                    last_refresh = Instant::now();
                } 
            }
            _ => ()
        }
    });
}

fn draw_loading_border(frame_buffer: &mut[u8]) {
    let mut random = rand::thread_rng();
    let mut rgb_color: (u8, u8, u8) = (0,0,0);
    rgb_color.0 = random.gen_range(0..255);
    rgb_color.1 = random.gen_range(0..255);
    rgb_color.2 = random.gen_range(0..255);

    let mut screen_pixel_count: u32 = 0;
    let mut line_count: u32 = 0;

    for pixel in frame_buffer.chunks_exact_mut(4) {
        pixel[3] = 255;

        if screen_pixel_count < BORDER || screen_pixel_count > WIDTH - BORDER || line_count < BORDER || line_count > HEIGHT - BORDER {
            if (line_count % (HEIGHT/48) == 0) && screen_pixel_count == 0 {
                rgb_color.0 = random.gen_range(0..255);
                rgb_color.1 = random.gen_range(0..255);
                rgb_color.2 = random.gen_range(0..255);
            }

            pixel[0] = rgb_color.0;
            pixel[1] = rgb_color.1;
            pixel[2] = rgb_color.2;
        }
    
        screen_pixel_count += 1;

        if screen_pixel_count == WIDTH {
            line_count += 1;
            screen_pixel_count = 0;
        }
    }
}

fn draw_background(frame_buffer: &mut[u8]) {

    let mut screen_pixel_count: u32 = 0;
    let mut line_count: u32 = 0;

    for pixel in frame_buffer.chunks_exact_mut(4) {

        if screen_pixel_count >= BORDER && screen_pixel_count <= WIDTH - BORDER && line_count >= BORDER && line_count <= HEIGHT - BORDER {
            pixel[0] = BKG_COLOR.0;
            pixel[1] = BKG_COLOR.1;
            pixel[2] = BKG_COLOR.2;
        }

        screen_pixel_count += 1;

        if screen_pixel_count == WIDTH {
            line_count += 1;
            screen_pixel_count = 0;
        }
    }
}

fn apply_crt_effect(frame_buffer: &mut[u8]) {

    let mut screen_pixel_count: u32 = 0;
    let mut line_count: u32 = 0;
    let mut sub_pixel_count: u32 = 0;

    for pixel in frame_buffer.chunks_exact_mut(4) {

        match sub_pixel_count {
            0 => {
                pixel[1] = 0;
                pixel[2] = 0;
            },
            1 => {
                pixel[0] = 0;
                pixel[2] = 0;
            },
            2 => {
                pixel[0] = 0;
                pixel[1] = 0;
            },
            3_u32..=u32::MAX => {}
        }
        
        if line_count % 4 == 0 {
            match sub_pixel_count {
                0 => {
                    pixel[0] = if pixel[0] <= 20 {0} else {pixel[0] - 20};
                    pixel[1] = 0;
                    pixel[2] = 0;
                },
                1 => {
                    pixel[0] = 0;
                    pixel[1] = if pixel[1] <= 20 {0} else {pixel[1] - 20};
                    pixel[2] = 0;
                },
                2 => {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = if pixel[2] <= 20 {0} else {pixel[2] - 20};
                },
                3_u32..=u32::MAX => {}
            }
        }

        screen_pixel_count += 1;
        sub_pixel_count = if sub_pixel_count == 2 {0} else {sub_pixel_count + 1};

        if screen_pixel_count == WIDTH {
            line_count += 1;
            screen_pixel_count = 0;
            sub_pixel_count = 0;
        }
    }
}

struct Position {
    x: u32, 
    y: u32
}

struct Sprite {
    position: Position,
    data: [u8; 64*3]
}

impl Sprite {

    fn new() -> Sprite {

        let position = Position {
            x: 0,
            y: 0
        }; 

        Sprite {
            position: position,
            data: [0;64*3] 
        }
    }
}

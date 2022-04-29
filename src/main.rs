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

fn main()-> Result<(), Error> 
{
    let event_loop = EventLoop::new();
    let builder = WindowBuilder::new()
        .with_decorations(true)
        .with_inner_size(PhysicalSize::new(WIDTH, HEIGHT))
        .with_title("Yay, une fenêtre !");

    let window = builder.build(&event_loop).expect("Window creation failed !");
    
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut last_refresh: Instant = Instant::now();
    let mut sec_counter: Instant = Instant::now();
    let mut apply_effect: bool = false;

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
                if last_refresh.elapsed().as_millis() >= 33
                {
                    //let frame = pixels.get_frame();
                    draw(pixels.get_frame(), apply_effect);
                    pixels.render().expect("Pixels render oups");
                    window.request_redraw();
                    last_refresh = Instant::now();
                } 
            }
            _ => ()
        }
        if sec_counter.elapsed().as_millis() >= 1000
        {
            apply_effect = !apply_effect;
            sec_counter = Instant::now();
        }
    });
}

fn draw(frame_buffer: &mut[u8], apply_effect: bool)
{
    let last_refresh: Instant = Instant::now();

    let mut random = rand::thread_rng();
    let chunk_size: u32 = WIDTH * (HEIGHT/48);
    
    let mut rgb_color: (u8, u8, u8) = (0,0,0);
    rgb_color.0 = random.gen_range(0..255);
    rgb_color.1 = random.gen_range(0..255);
    rgb_color.2 = random.gen_range(0..255);

    let mut pixel_count: u32 = 0;
    let mut line_count: u32 = 0;
    let mut sub_pixel_count = 0;

    for pixel in frame_buffer.chunks_exact_mut(4) 
    {
        pixel[3] = 255;

        if pixel_count % chunk_size == 0 
        {
            rgb_color.0 = random.gen_range(0..255);
            rgb_color.1 = random.gen_range(0..255);
            rgb_color.2 = random.gen_range(0..255);
        }

        if apply_effect
        {
            match sub_pixel_count {
                0 => 
                {
                    pixel[0] = rgb_color.0;
                    pixel[1] = 0;
                    pixel[2] = 0;
                },
                1 => 
                {
                    pixel[0] = 0;
                    pixel[1] = rgb_color.1;
                    pixel[2] = 0;
                },
                2 =>
                {
                    pixel[0] = 0;
                    pixel[1] = 0;
                    pixel[2] = rgb_color.2;
                },
                3_u32..=u32::MAX => {}
            }
            
            if line_count % 4 == 0
            {
                match sub_pixel_count {
                    0 => 
                    {
                        pixel[0] = if rgb_color.0 <= 20 {0} else {rgb_color.0 - 20};
                        pixel[1] = 0;
                        pixel[2] = 0;
                    },
                    1 => 
                    {
                        pixel[0] = 0;
                        pixel[1] = if rgb_color.1 <= 20 {0} else {rgb_color.1 - 20};
                        pixel[2] = 0;
                    },
                    2 =>
                    {
                        pixel[0] = 0;
                        pixel[1] = 0;
                        pixel[2] = if rgb_color.2 <= 20 {0} else {rgb_color.2 - 20};
                    },
                    3_u32..=u32::MAX => {}
                }
            }
        }
        else
        {
            pixel[0] = rgb_color.0;
            pixel[1] = rgb_color.1;
            pixel[2] = rgb_color.2;
        }
        
        pixel_count += 1;
        sub_pixel_count = if sub_pixel_count == 2 { 0 } else { sub_pixel_count + 1 };

        if pixel_count % WIDTH == 0
        {
            line_count += 1;
            sub_pixel_count = 0;
        }
    }

    println!("draw time {}us", last_refresh.elapsed().as_micros());
}

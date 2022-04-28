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

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

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
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                let frame = pixels.get_frame();
                draw(frame);
                pixels.render().expect("Pixels render oups");
                window.request_redraw(); 
            }
            _ => ()
        }
    });
}

fn draw(frame_buffer: &mut[u8])
{
    let last_refresh: Instant = Instant::now();

    let mut random = rand::thread_rng();
    let chunk_size: u32 = WIDTH * (HEIGHT/24);
    let mut pixel_count: u32 = 0;
    let mut r: u8 = random.gen_range(0..255);
    let mut g: u8 = random.gen_range(0..255);
    let mut b: u8 = random.gen_range(0..255);
    let mut line_count: u32 = 0;

    for pixel in frame_buffer.chunks_exact_mut(4) 
    {
        pixel[0] = r;
        pixel[1] = g;
        pixel[2] = b;
        pixel[3] = 255;

        if pixel_count % chunk_size == 0 
        {
            r = random.gen_range(0..255);
            g = random.gen_range(0..255);
            b = random.gen_range(0..255);
        } 
        
        if line_count % 2 == 0
        {
            pixel[0] = pixel[0]/2;
            pixel[1] = pixel[1]/2;
            pixel[2] = pixel[2]/2;
        }

        pixel_count = pixel_count + 1;

        if pixel_count % WIDTH == 0
        {
            line_count = line_count + 1
        }
    }
    println!("draw time {}ms", last_refresh.elapsed().as_millis());
}

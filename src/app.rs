use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop,EventLoopProxy}};
use crate::virtual_frame_buffer::VirtualFrameBuffer;

pub struct App {
    name: String,
    pub updating: bool,
    pub started: bool,
    pub ended: bool,
    pub drawing: bool,
    pid: usize
}

impl App {
    pub fn new(name: String, pid: usize) -> App {
        App {
            name,
            pid,
            updating: true,
            started: false,
            ended: false,
            drawing: true
        }
    }

    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_pid(&self) -> usize {
        return self.pid;
    }
}   

pub trait Update {
    fn update(&mut self, character_received: Option<char>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow>;
    fn start(&mut self);
    fn end(&mut self);
}

pub trait Draw {
    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer);
}
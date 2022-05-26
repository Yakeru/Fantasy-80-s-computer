use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use crate::virtual_frame_buffer::VirtualFrameBuffer; 
use crate::text_layer::TextLayerChar;

#[derive(Clone)]
pub struct ProcessResponse {
    pub event: Option<ControlFlow>,
    pub message: Option<String>
}

pub enum ProcessMessage {
    Message(String, Option<ControlFlow>),
    StartApp(String),
    QuitApp(String),
    KillApp(String),
    GiveMeFocus,
}

impl ProcessResponse {

    pub fn new() -> ProcessResponse {
        ProcessResponse {
            event: None,
            message: None
        }
    }

    pub fn set_message(&mut self, string: String) {
        self.message = Some(string);
    }
}

pub trait Process {
    fn update(&mut self, character_received: Option<char>, key_pressed_os: Option<VirtualKeyCode>, key_released: Option<VirtualKeyCode>) -> ProcessResponse;
    fn start(&mut self);
    fn end(&mut self);
    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer);
    fn get_name(&self) -> &str;
    fn set_state(&mut self, updating: bool, drawing: bool);
    fn get_state(&self) -> (bool, bool);
}
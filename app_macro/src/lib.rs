use winit::{event::{VirtualKeyCode, KeyboardInput}, event_loop::ControlFlow};
use virtual_frame_buffer::*;

pub trait AppMacro {
    fn start(&mut self);
    fn end(&mut self);
    fn get_name(&self) -> &str;
    fn set_state(&mut self, updating: bool, drawing: bool);
    fn get_state(&self) -> (bool, bool);
    fn update(&mut self, keybord_input: Option<KeyboardInput>, char_received: Option<char>) -> AppResponse;
    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer);
}

#[derive(Clone)]
pub struct AppResponse {
    pub event: Option<ControlFlow>,
    pub message: Option<String>,
}

pub enum AppMessage {
    Message(String, Option<ControlFlow>),
    StartApp(String),
    QuitApp(String),
    KillApp(String),
    GiveMeFocus,
}

impl AppResponse {
    pub fn new() -> AppResponse {
        AppResponse {
            event: None,
            message: None,
        }
    }

    pub fn set_message(&mut self, string: String) {
        self.message = Some(string);
    }
}

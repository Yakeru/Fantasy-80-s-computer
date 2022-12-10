use clock::Clock;
use winit::{event::KeyboardInput, event_loop::ControlFlow};
use virtual_frame_buffer::*;

pub trait AppMacro {
    fn get_name(&self) -> &str;
    fn set_state(&mut self, updating: bool, drawing: bool);
    fn get_state(&self) -> (bool, bool);
    fn update(&mut self, app_inputs: AppInputs, virtual_frame_buffer: &mut VirtualFrameBuffer) -> Option<AppResponse>;
    fn draw(&mut self, app_inputs: AppInputs, virtual_frame_buffer: &mut VirtualFrameBuffer);
}

#[derive(Clone)]
pub struct AppResponse {
    pub event: Option<ControlFlow>,
    pub message: Option<String>,
}

#[derive(Clone, Copy)]
pub struct AppInputs {
    pub keyboard_input: Option<KeyboardInput>,
    pub char_received: Option<char>,
    pub mouse_move_delta: (f64, f64),
    pub system_clock: Clock
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

    pub fn get_message(&self) -> &Option<String> {
        &self.message
    }
}

use clock::Clock;
use winit::{event::KeyboardInput, event_loop::ControlFlow};
use virtual_frame_buffer::*;
use winit_input_helper::WinitInputHelper;

pub trait AppMacro {
    fn get_name(&self) -> &str;
    fn set_state(&mut self, updating: bool, drawing: bool);
    fn get_state(&self) -> (bool, bool);
    fn update(&mut self, inputs: &WinitInputHelper, clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer) -> Option<AppResponse>;
    fn draw(&mut self, inputs: &WinitInputHelper, clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer);
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

// impl AppInputs {

//     pub fn test(&self) {
//         match self.keyboard_input {
//             Some(input) => {
//                 match input.virtual_keycode {
//                     Some(code) => (),
//                     None => ()
//                 }
//             },
//             None => ()
//         }
//     }

//     pub fn get_keyboard_input(&self) -> Option<KeyboardInput> {
//         self.keyboard_input
//     }

//     pub fn get_char_received(&self) -> Option<char> {
//         self.char_received
//     }

//     pub fn set_keyboard_input(&mut self, keyboardInput: KeyboardInput) {

//     }

// }

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

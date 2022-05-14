use winit::{event::VirtualKeyCode,event_loop::ControlFlow};
use crate::virtual_frame_buffer::VirtualFrameBuffer; 

pub trait Process {
    fn update(&mut self, character_received: Option<char>, key_pressed_os: Option<VirtualKeyCode>, key_released: Option<VirtualKeyCode>) -> Option<ControlFlow>;
    fn start(&mut self);
    fn end(&mut self);
    fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer);
    fn get_name(&mut self) -> &str;
    fn set_state(&mut self, updating: bool, drawing: bool);
    fn get_state(&mut self) -> (bool, bool);
}
use clock::Clock;
use display_controller::*;
use winit::event_loop::ControlFlow;
use winit_input_helper::WinitInputHelper;

#[derive(Debug, PartialEq, Eq)]
pub enum AppStatus {
    Stopped,
    Running,
    Background,
}

pub trait AppMacro {
    fn get_name(&self) -> &str;
    fn set_state(&mut self, status: AppStatus);
    fn get_state(&self) -> &AppStatus;
    fn exec_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        clock: &Clock,
        display_controller: &mut DisplayController,
    ) -> Option<AppResponse>;
    fn update(&mut self, inputs: Option<&WinitInputHelper>, clock: &Clock) -> Option<AppResponse>;
    fn draw(&mut self, clock: &Clock, display_controller: &mut DisplayController);
}

#[derive(Clone)]
pub struct AppResponse {
    pub event: Option<ControlFlow>,
    pub message: Option<String>,
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

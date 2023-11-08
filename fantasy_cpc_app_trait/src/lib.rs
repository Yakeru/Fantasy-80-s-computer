use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::DisplayController;
use winit::{event::VirtualKeyCode, event_loop::ControlFlow};
use winit_input_helper::WinitInputHelper;

#[derive(Debug, PartialEq, Eq)]
pub enum AppStatus {
    Stopped,
    Running,
    Background,
}

pub trait FantasyCpcApp {
    fn get_name(&self) -> &str;

    fn get_state(&self) -> &AppStatus;
    fn set_state(&mut self, state: AppStatus);

    fn get_initialized(&self) -> bool;
    fn set_initialized(&mut self, is_initialized: bool);

    fn get_enable_autoescape(&self) -> bool;
    // fn set_enable_autoescape(&mut self, enable_auto_escape: bool);

    fn init_app(&mut self, system_clock: &Clock, display_controller: &mut DisplayController);

    fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        clock: &Clock,
    ) -> Option<AppResponse>;

    fn draw_app(&mut self, clock: &Clock, display_controller: &mut DisplayController);

    fn change_state(&mut self, new_state: AppStatus) {
        match new_state {
            AppStatus::Stopped => {
                self.set_initialized(false);
                self.set_state(AppStatus::Stopped);
            }
            _ => self.set_state(new_state),
        }
    }

    fn exec_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        system_clock: &Clock,
        display_controller: &mut DisplayController,
    ) -> Option<AppResponse> {
        match self.get_state() {
            AppStatus::Stopped => None,
            AppStatus::Running => {
                if !self.get_initialized() {
                    self.init_app(system_clock, display_controller);
                    self.set_initialized(true);
                }
                let app_response = self.update(inputs, system_clock);
                self.draw(system_clock, display_controller);
                return app_response;
            }
            AppStatus::Background => {
                if !self.get_initialized() {
                    self.init_app(system_clock, display_controller);
                    self.set_initialized(true);
                }
                return self.update(None, system_clock);
            }
        }
    }

    fn update(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        system_clock: &Clock,
    ) -> Option<AppResponse> {
        // Implementing default behaviour when ESCAPE key is pressed in app
        // Applied only if enable_auto_escape is set to true in app.
        if inputs.is_some() && self.get_enable_autoescape() {
            if inputs.unwrap().key_released(VirtualKeyCode::Escape) {
                self.set_state(AppStatus::Stopped);
                self.set_initialized(false);
            }
        }

        return self.update_app(inputs, system_clock);
    }

    fn draw(&mut self, system_clock: &Clock, display_controller: &mut DisplayController) {
        self.draw_app(system_clock, display_controller);
    }
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

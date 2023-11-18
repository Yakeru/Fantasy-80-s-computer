use std::fmt;

use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::DisplayController;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

#[derive(Debug, PartialEq, Eq)]
pub enum AppStatus {
    Stopped,
    Running,
    Background,
}

pub struct FantasyCppAppDefaultParams {
    name: String,
    status: AppStatus,
    initialized: bool,
    autoescape_enabled: bool,
}

impl FantasyCppAppDefaultParams {
    pub fn new(name: String, autoescape_enabled: bool) -> FantasyCppAppDefaultParams {
        FantasyCppAppDefaultParams {
            name,
            status: AppStatus::Stopped,
            initialized: false,
            autoescape_enabled,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_status(&self) -> &AppStatus {
        &self.status
    }

    pub fn change_status(&mut self, new_state: AppStatus) {
        match new_state {
            AppStatus::Stopped => {
                self.initialized = false;
                self.status = AppStatus::Stopped;
            }
            _ => self.status = new_state,
        }
    }

    pub fn get_initialized(&self) -> bool {
        self.initialized
    }

    pub fn set_initialized(&mut self, is_initialized: bool) {
        self.initialized = is_initialized
    }

    pub fn get_enable_autoescape(&self) -> bool {
        self.autoescape_enabled
    }
}

pub trait FantasyCpcApp {
    fn get_app_params(&mut self) -> &mut FantasyCppAppDefaultParams;

    fn init_app(&mut self, system_clock: &Clock, display_controller: &mut DisplayController);

    fn update_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        messages: Option<Vec<AppMessage>>,
        clock: &Clock,
    ) -> Option<Vec<AppMessage>>;

    fn draw_app(&mut self, clock: &Clock, display_controller: &mut DisplayController);

    fn exec_app(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        messages: Option<Vec<AppMessage>>,
        system_clock: &Clock,
        display_controller: &mut DisplayController,
    ) -> Option<Vec<AppMessage>> {
        match self.get_app_params().get_status() {
            AppStatus::Stopped => None,
            AppStatus::Running => {
                if !self.get_app_params().get_initialized() {
                    self.init_app(system_clock, display_controller);
                    self.get_app_params().set_initialized(true);
                }
                let app_response = self.update(inputs, messages, system_clock);
                self.draw(system_clock, display_controller);
                app_response
            }
            AppStatus::Background => {
                if !self.get_app_params().get_initialized() {
                    self.init_app(system_clock, display_controller);
                    self.get_app_params().set_initialized(true);
                }
                self.update(None, messages, system_clock)
            }
        }
    }

    fn update(
        &mut self,
        inputs: Option<&WinitInputHelper>,
        messages: Option<Vec<AppMessage>>,
        system_clock: &Clock,
    ) -> Option<Vec<AppMessage>> {
        // Implementing default behaviour when ESCAPE key is pressed in app
        // Applied only if enable_auto_escape is set to true in app.
        if inputs.is_some()
            && self.get_app_params().get_enable_autoescape()
            && inputs.unwrap().key_released(VirtualKeyCode::Escape)
        {
            self.get_app_params().change_status(AppStatus::Stopped);
            self.get_app_params().set_initialized(false);
        }

        self.update_app(inputs, messages, system_clock)
    }

    fn draw(&mut self, system_clock: &Clock, display_controller: &mut DisplayController) {
        self.draw_app(system_clock, display_controller);
    }
}

#[derive(Clone)]
pub enum AppMessage {
    System(String),
    Standard(String),
    Highlight(String),
    Warning(String),
    Error(String),
    Critical(String),
}

impl fmt::Display for AppMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppMessage::System(text) => write!(f, "SYSTEM: {}", text),
            AppMessage::Standard(text) => write!(f, "{}", text),
            AppMessage::Highlight(text) => write!(f, "/!\\ {}", text),
            AppMessage::Warning(text) => write!(f, "WARNING: {}", text),
            AppMessage::Error(text) => write!(f, "ERROR: {}", text),
            AppMessage::Critical(text) => write!(f, "CRITICAL: {}", text),
        }
    }
}

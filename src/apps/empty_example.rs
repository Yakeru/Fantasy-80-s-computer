use app_trait::{AppStatus, FantasyCpcApp};

pub struct Empty {
    enable_auto_escape: bool,
    name: String,
    status: AppStatus,
    initialized: bool,
}

impl Empty {
    pub fn _new() -> Empty {
        Self {
            enable_auto_escape: true,
            name: "Empty".to_string(),
            status: AppStatus::Stopped,
            initialized: false,
        }
    }
}

impl FantasyCpcApp for Empty {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_state(&self) -> &app_trait::AppStatus {
        &self.status
    }

    fn set_state(&mut self, state: app_trait::AppStatus) {
        self.status = state;
    }

    fn get_initialized(&self) -> bool {
        self.initialized
    }

    fn set_initialized(&mut self, is_initialized: bool) {
        self.initialized = is_initialized
    }

    fn get_enable_autoescape(&self) -> bool {
        self.enable_auto_escape
    }

    fn set_enable_autoescape(&mut self, enable_auto_escape: bool) {
        self.enable_auto_escape = enable_auto_escape
    }

    fn init_app(
        &mut self,
        _system_clock: &clock::Clock,
        _display_controller: &mut display_controller::DisplayController,
    ) {
    }

    fn update_app(
        &mut self,
        _inputs: Option<&winit_input_helper::WinitInputHelper>,
        _clock: &clock::Clock,
    ) -> Option<app_trait::AppResponse> {
        None
    }

    fn draw_app(
        &mut self,
        _clock: &clock::Clock,
        _display_controller: &mut display_controller::DisplayController,
    ) {
    }
}

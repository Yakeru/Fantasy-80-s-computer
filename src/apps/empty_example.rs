use app_macro_derive::AppMacro;

#[derive(AppMacro)]
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

    fn init_app(&mut self, _clock: &Clock, _dc: &mut DisplayController) {}

    fn update_app(
        &mut self,
        _inputs: Option<&WinitInputHelper>,
        _clock: &Clock,
    ) -> Option<AppResponse> {
        None
    }

    fn draw_app(&mut self, _clock: &Clock, _display_controller: &mut DisplayController) {}
}

use app_macro_derive::AppMacro;

#[derive(AppMacro)]
pub struct Empty {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
}

impl Empty {

    pub fn _new() -> Empty {
        Self { enable_auto_escape: true, name: "Empty".to_string(), updating: false, drawing: false, initialized: false }
    }
    
    pub fn init_app(&mut self, _clock: &Clock, _dc: &mut DisplayController) {}

    pub fn update_app(
        &mut self,
        _inputs: Option<&WinitInputHelper>,
        _clock: &Clock,
        _display_controller: &mut DisplayController,
    ) -> Option<AppResponse> {

        None
    }

    pub fn draw_app(
        &mut self,
        _clock: &Clock,
        _display_controller: &mut DisplayController,
    ) {}
}
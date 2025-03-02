use fantasy_cpc_app::{AppResponse, FantasyCpcApp, FantasyCppAppDefaultParams};
use fantasy_cpc_clock::Clock;
use fantasy_cpc_display_controller::DisplayController;

pub struct Empty {
    app_params: FantasyCppAppDefaultParams,
}

impl Empty {
    pub fn _new() -> Empty {
        Self {
            app_params: FantasyCppAppDefaultParams::new(String::from("Empty"), true),
        }
    }
}

impl FantasyCpcApp for Empty {
    fn get_app_params(&mut self) -> &mut FantasyCppAppDefaultParams {
        &mut self.app_params
    }

    fn init_app(&mut self, _system_clock: &Clock, _display_controller: &mut DisplayController) {}

    fn update_app(
        &mut self,
        _inputs: Option<&winit_input_helper::WinitInputHelper>,
        _clock: &Clock,
    ) -> Option<AppResponse> {
        None
    }

    fn draw_app(&mut self, _clock: &Clock, _display_controller: &mut DisplayController) {}
}

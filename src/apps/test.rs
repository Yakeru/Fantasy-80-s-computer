use app_macro_derive::AppMacro;

#[derive(AppMacro)]
pub struct Test {
    enable_auto_escape: bool,
    name: String,
    updating: bool,
    drawing: bool,
    initialized: bool,
}

impl Test {
    
    pub fn init_app(&mut self, _virtual_frame_buffer: &mut VirtualFrameBuffer) {}

    pub fn update_app(
        &mut self,
        inputs: &WinitInputHelper,
        _clock: &Clock,
        virtual_frame_buffer: &mut VirtualFrameBuffer
    ) -> Option<AppResponse> {

        return None;
    }

    pub fn draw_app(&mut self, inputs: &WinitInputHelper, clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer) {}
}
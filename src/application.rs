pub struct Application {
    name: String
}

impl Application {

    pub fn new(name: String) -> Application {

        Application {
            name
        }
    }

    pub fn handle_event(&mut self, event: &winit::event::WindowEvent) {

    }

}

pub trait RenderToTextLayer {
    fn render_to_text_layer(&self);
}
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(AppMacro)]
pub fn app_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_app_macro(&ast)
}

fn impl_app_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {

        use winit::event::VirtualKeyCode;
        use winit::event_loop::*;
        use winit_input_helper::*;
        use clock::Clock;
        use display_controller::*;
        use display_controller::config::*;
        use display_controller::color_palettes::*;
        use display_controller::text_layer::*;
        use app_macro::*;

        impl AppMacro for #name {

            fn get_name(&self) -> &str {
                &self.name
            }

            fn set_state(&mut self, updating: bool, drawing: bool) {
                self.updating = updating;
                self.drawing = drawing;

                if self.drawing {self.updating = true}
                if !self.updating {self.drawing = false}
            }

            fn get_state(&self) -> (bool, bool) {
                (self.updating, self.drawing)
            }

            fn update(&mut self, inputs: Option<&WinitInputHelper>, system_clock: &Clock, display_controller: &mut DisplayController) -> Option<AppResponse> {
                
                if !self.initialized {
                    self.init_app(system_clock, display_controller);
                    self.initialized = true;
                }

                // Implementing default behaviour when ESCAPE key is pressed in app
                // Applied only if enable_auto_escape is set to true in app.
                if inputs.is_some() && self.enable_auto_escape {
                    if inputs.unwrap().key_released(VirtualKeyCode::Escape) {
                        self.set_state(false, false);
                        self.initialized = false;
                        display_controller.set_brightness(255);
                    }
                }
                
                return self.update_app(inputs, system_clock, display_controller);
            }
            
            fn draw(&mut self, system_clock: &Clock, display_controller: &mut DisplayController) {
                self.draw_app(system_clock, display_controller);
            }
        }
    };
    gen.into()
}

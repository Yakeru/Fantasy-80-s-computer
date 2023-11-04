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
        use app_macro::{*, AppStatus::*};

        impl AppMacro for #name {

            fn get_name(&self) -> &str {
                &self.name
            }

            fn set_state(&mut self, status: AppStatus) {
                self.status = status;

                if self.status == AppStatus::Stopped {
                    self.initialized = false;
                }
            }

            fn get_state(&self) -> &AppStatus {
                &self.status
            }

            fn exec_app(&mut self, inputs: Option<&WinitInputHelper>, system_clock: &Clock, display_controller: &mut DisplayController) -> Option<AppResponse> {

                match self.status {
                    Stopped => (None),
                    Running => {
                        if !self.initialized {
                            self.init_app(system_clock, display_controller);
                            self.initialized = true;
                        }
                        let app_response = self.update(inputs, system_clock);
                        self.draw(system_clock, display_controller);
                        return app_response;
                    },
                    Background => {
                        if !self.initialized {
                            self.init_app(system_clock, display_controller);
                            self.initialized = true;
                        }
                        return self.update(None, system_clock);
                    }
                }
            }

            fn update(&mut self, inputs: Option<&WinitInputHelper>, system_clock: &Clock) -> Option<AppResponse> {

                // Implementing default behaviour when ESCAPE key is pressed in app
                // Applied only if enable_auto_escape is set to true in app.
                if inputs.is_some() && self.enable_auto_escape {
                    if inputs.unwrap().key_released(VirtualKeyCode::Escape) {
                        self.set_state(Stopped);
                        self.initialized = false;
                    }
                }

                return self.update_app(inputs, system_clock);
            }

            fn draw(&mut self, system_clock: &Clock, display_controller: &mut DisplayController) {
                self.draw_app(system_clock, display_controller);
            }
        }
    };
    gen.into()
}

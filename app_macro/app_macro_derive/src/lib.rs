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

            fn update(&mut self, inputs: &WinitInputHelper, system_clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer) -> Option<AppResponse> {
                
                if !self.initialized {
                    self.init_app(virtual_frame_buffer);
                    self.initialized = true;
                }

                // Implementing default behaviour when ESCAPE key is pressed in app
                // Applied only if enable_auto_escape is set to true in app.
                if self.enable_auto_escape {
                    if inputs.key_released(VirtualKeyCode::Escape) {
                        self.set_state(false, false);
                    }
                }
                
                return self.update_app(inputs, system_clock, virtual_frame_buffer);
            }
            
            fn draw(&mut self, inputs: &WinitInputHelper, system_clock: &Clock, virtual_frame_buffer: &mut VirtualFrameBuffer) {
                self.draw_app(inputs, system_clock, virtual_frame_buffer);
            }
        }
    };
    gen.into()
}

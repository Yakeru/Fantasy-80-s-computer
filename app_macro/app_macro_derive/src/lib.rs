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

            fn start(&mut self) {
                self.started = true;
                self.drawing = true;
                self.updating = true;
                self.ended = false;
            }

            fn end(&mut self) {
                self.started = false;
                self.drawing = false;
                self.updating = false;
                self.ended = true;
            }

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

            fn update(&mut self, app_message: AppMessage, virtual_frame_buffer: &mut VirtualFrameBuffer) -> Option<AppResponse> {
                
                // Implementing default behaviour when ESCAPE key is pressed in app
                // Ignore for shell
                if self.enable_auto_escape {
                    match app_message.keyboard_input {
                        Some(key) => {
                            match(key.virtual_keycode) {
                                Some(keycode) => {
                                    if keycode == VirtualKeyCode::Escape && key.state == ElementState::Released {
                                        self.end()
                                    }
                                },
                                None => ()
                            } 
                        },
                        None => ()
                    }
                }
                
                return self.update_app(app_message, virtual_frame_buffer);
            }
            
            fn draw(&mut self, virtual_frame_buffer: &mut VirtualFrameBuffer) {
                self.draw_app(virtual_frame_buffer);
            }
        }
    };
    gen.into()
}

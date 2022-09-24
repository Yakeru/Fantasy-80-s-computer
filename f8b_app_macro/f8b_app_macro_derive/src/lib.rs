extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(F8bAppMacro)]
pub fn f8b_app_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_f8b_app_macro(&ast)
}

fn impl_f8b_app_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl F8bAppMacro for #name {

            // fn hello_macro() {
            //     println!("Hello, Macro! My name is {}!", stringify!(#name));
            // }

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
        
                if drawing {self.updating = true}
                if !updating {self.drawing = false}
            }
        
            fn get_state(&self) -> (bool, bool) { 
                (self.updating, self.drawing)
            }
        }
    };
    gen.into()
}
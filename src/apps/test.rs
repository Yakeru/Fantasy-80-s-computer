use f8b_app_macro::F8bAppMacro;
use f8b_app_macro_derive::F8bAppMacro;

#[derive(F8bAppMacro)]
pub struct Test {
    name: String,
    updating: bool,
    drawing: bool,
    started: bool,
    ended: bool
}

impl Test {
    pub fn new() -> Test {
        Test {
            name: String::from("Test"),
            updating: false,
            drawing: false,
            started: false,
            ended: false
        }
    }
}
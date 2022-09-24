pub trait F8bAppMacro {
    fn start(&mut self);
    fn end(&mut self);
    fn get_name(&self) -> &str;
    fn set_state(&mut self, updating: bool, drawing: bool);
    fn get_state(&self) -> (bool, bool);
}
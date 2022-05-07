//Struct describing all the settings one character can have in text mode
//"flipp" tells the renderer to flip the color and background of that character
//"blink" tells the renderer to automatically flip the color and background of that character at a set interval, useful for blinking warning messages
pub struct TextModeChar {
    c: char,
    background_color: u8,
    color: u8,
    flipp: bool,
    blink: bool
}

//The virtual text mode buffer, width and height are expressed in characters
pub struct VirtualTextLayerFrameBuffer {
    width: u32,
    height: u32,
    frame: Vec<TextModeChar>
}
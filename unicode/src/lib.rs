pub const ENTER: char = '\u{000D}'; //Unicode for carriage return. 000A is line feed
pub const ESCAPE: char = '\u{001B}';
pub const TAB: char = '\u{0009}';
#[cfg(target_os = "macos")]
pub const BACKSPACE: char = '\u{9003}';
#[cfg(target_os = "windows")]
pub const BACKSPACE: char = '\u{0008}';
#[cfg(target_os = "linux")]
pub const BACKSPACE: char = '\u{0008}';
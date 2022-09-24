pub const ENTER: char = '\u{000D}';
pub const ESCAPE: char = '\u{001B}';
#[cfg(target_os = "macos")]
pub const BACKSPACE: char = '\u{9003}';
#[cfg(target_os = "windows")]
pub const BACKSPACE: char = '\u{0008}';


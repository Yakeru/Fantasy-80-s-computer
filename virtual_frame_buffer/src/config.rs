// 4K
// pub const WIDTH: usize = 2560;
// pub const HEIGHT: usize = 1920;
// pub const UPSCALE: usize = 6;
// pub const SCAN_LINE_STRENGTH: u8 = 100;
// pub const VIRTUAL_WIDTH: usize = 426;
// pub const VIRTUAL_HEIGHT: usize = 320;
// pub const TEXT_COLUMNS: usize = 46;
// pub const TEXT_ROWS: usize = 34;

// QHD
pub const WIDTH: usize = 2048;
pub const HEIGHT: usize = 1536;
pub const UPSCALE: usize = 6;
pub const SCAN_LINE_STRENGTH: u8 = 100;
pub const VIRTUAL_WIDTH: usize = 340;
pub const VIRTUAL_HEIGHT: usize = 256;
pub const OVERSCAN_V: usize = 8;
pub const OVERSCAN_H: usize = 10;
pub const TEXT_COLUMNS: usize = 40; //320px
pub const TEXT_ROWS: usize = 30; //240px

// 1080p
// pub const WIDTH: usize = 1280;
// pub const HEIGHT: usize = 960;
// pub const UPSCALE: usize = 3;
// pub const SCAN_LINE_STRENGTH: u8 = 100;
// pub const VIRTUAL_WIDTH: usize = 426;
// pub const VIRTUAL_HEIGHT: usize = 320;
// pub const TEXT_COLUMNS: usize = 44;
// pub const TEXT_ROWS: usize = 32;

// No upscaling
// pub const WIDTH: usize = 426;
// pub const HEIGHT: usize = 320;
// pub const UPSCALE: usize = 1;
// pub const SCAN_LINE_STRENGTH: u8 = 100;
// pub const VIRTUAL_WIDTH: usize = 426;
// pub const VIRTUAL_HEIGHT: usize = 320;
// pub const TEXT_COLUMNS: usize = 44;
// pub const TEXT_ROWS: usize = 32;

/* The following code is an experiment to generate a config struct, automatically selected
according to the resolution of the screen, but since I'm using arrays all over the place
and Rust doesnt allow non const values to initialize arrays, the refactoring to implement
the auto-config was a bit to much work for now. Will get back to it later */

// // 4K
// pub const FOUR_K_WIDTH: usize = 2560;
// pub const FOUR_K_HEIGHT: usize = 1920;
// pub const FOUR_K_UPSCALE: usize = 6;
// pub const FOUR_K_SCAN_LINE_STRENGTH: u8 = 100;
// pub const FOUR_K_VIRTUAL_WIDTH: usize = 426;
// pub const FOUR_K_VIRTUAL_HEIGHT: usize = 320;
// pub const FOUR_K_TEXT_COLUMNS: usize = 46;
// pub const FOUR_K_TEXT_ROWS: usize = 34;

// // QHD
// pub const QHD_WIDTH: usize = 2048;
// pub const QHD_HEIGHT: usize = 1536;
// pub const QHD_UPSCALE: usize = 6;
// pub const QHD_SCAN_LINE_STRENGTH: u8 = 100;
// pub const QHD_VIRTUAL_WIDTH: usize = 340;
// pub const QHD_VIRTUAL_HEIGHT: usize = 256;
// pub const QHD_TEXT_COLUMNS: usize = 40;
// pub const QHD_TEXT_ROWS: usize = 30;

// // 1080p
// pub const FHD_WIDTH: usize = 1280;
// pub const FHD_HEIGHT: usize = 960;
// pub const FHD_UPSCALE: usize = 3;
// pub const FHD_SCAN_LINE_STRENGTH: u8 = 100;
// pub const FHD_VIRTUAL_WIDTH: usize = 426;
// pub const FHD_VIRTUAL_HEIGHT: usize = 320;
// pub const FHD_TEXT_COLUMNS: usize = 44;
// pub const FHD_TEXT_ROWS: usize = 32;

// // No upscaling
// pub const DEFAULT_WIDTH: usize = 426;
// pub const DEFAULT_HEIGHT: usize = 320;
// pub const DEFAULT_UPSCALE: usize = 1;
// pub const DEFAULT_SCAN_LINE_STRENGTH: u8 = 100;
// pub const DEFAULT_VIRTUAL_WIDTH: usize = 426;
// pub const DEFAULT_VIRTUAL_HEIGHT: usize = 320;
// pub const DEFAULT_TEXT_COLUMNS: usize = 44;
// pub const DEFAULT_TEXT_ROWS: usize = 32;

// pub struct DisplayConfig {
//     pub window_width: usize,
//     pub window_height: usize,
//     pub frame_width: usize,
//     pub frame_height: usize,
//     pub upscale_factor: usize,
//     pub scan_line_strength: u8,
//     pub text_columns: usize,
//     pub text_rows: usize
// }

// const fn auto_detect_mode(screen_width: i32, screen_height: i32) -> u8 {
//     match screen_width {
//         2560 => 3,
//         2048 => 2,
//         1920 => 1,
//         _ => 0
//     }
// }

// const fn get_config(mode: u8) -> DisplayConfig {
//     match mode {
//         // Default, no upscale, no crt effect
//         0 => {
//             DisplayConfig {
//                 window_width: DEFAULT_WIDTH,
//                 window_height: DEFAULT_HEIGHT,
//                 frame_width: DEFAULT_VIRTUAL_WIDTH,
//                 frame_height: DEFAULT_VIRTUAL_HEIGHT,
//                 upscale_factor: DEFAULT_UPSCALE,
//                 scan_line_strength: DEFAULT_SCAN_LINE_STRENGTH,
//                 text_columns: DEFAULT_TEXT_COLUMNS,
//                 text_rows: DEFAULT_TEXT_ROWS
//             }
//         },
//         // Full HD, upscaled x3
//         1 => {
//             DisplayConfig {
//                 window_width: FHD_WIDTH,
//                 window_height: FHD_HEIGHT,
//                 frame_width: FHD_VIRTUAL_WIDTH,
//                 frame_height: FHD_VIRTUAL_HEIGHT,
//                 upscale_factor: FHD_UPSCALE,
//                 scan_line_strength: FHD_SCAN_LINE_STRENGTH,
//                 text_columns: FHD_TEXT_COLUMNS,
//                 text_rows: FHD_TEXT_ROWS
//             }
//         },
//         //Quad HD, upscaled 6x
//         2 => {
//             DisplayConfig {
//                 window_width: QHD_WIDTH,
//                 window_height: QHD_HEIGHT,
//                 frame_width: QHD_VIRTUAL_WIDTH,
//                 frame_height: QHD_VIRTUAL_HEIGHT,
//                 upscale_factor: QHD_UPSCALE,
//                 scan_line_strength: QHD_SCAN_LINE_STRENGTH,
//                 text_columns: QHD_TEXT_COLUMNS,
//                 text_rows: QHD_TEXT_ROWS
//             }
//         },
//         // 4K, upscaled x6
//         3 => {
//             DisplayConfig {
//                 window_width: DEFAULT_WIDTH,
//                 window_height: DEFAULT_HEIGHT,
//                 frame_width: DEFAULT_VIRTUAL_WIDTH,
//                 frame_height: DEFAULT_VIRTUAL_HEIGHT,
//                 upscale_factor: DEFAULT_UPSCALE,
//                 scan_line_strength: DEFAULT_SCAN_LINE_STRENGTH,
//                 text_columns: DEFAULT_TEXT_COLUMNS,
//                 text_rows: DEFAULT_TEXT_ROWS
//             }
//         },
//         _ => {
//             DisplayConfig {
//                 window_width: FOUR_K_WIDTH,
//                 window_height: FOUR_K_HEIGHT,
//                 frame_width: FOUR_K_VIRTUAL_WIDTH,
//                 frame_height: FOUR_K_VIRTUAL_HEIGHT,
//                 upscale_factor: FOUR_K_UPSCALE,
//                 scan_line_strength: FOUR_K_SCAN_LINE_STRENGTH,
//                 text_columns: FOUR_K_TEXT_COLUMNS,
//                 text_rows: FOUR_K_TEXT_ROWS
//             }
//         }
//     }
// }

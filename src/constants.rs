use kit::*;

// grid
pub const BOARD_SIZE: i32 = 5;
pub const BOARD_SIZE_HALF: i32 = BOARD_SIZE / 2;
pub const BOARD_NUM_SPACES: usize = BOARD_SIZE as usize * 3;

pub const BOARD_CELL_WIDTH_PX: f32 = 92.0;
pub const BOARD_CELL_WIDTH_PX_HALF: f32 = BOARD_CELL_WIDTH_PX / 2.0;
pub const BOARD_CELL_HEIGHT_PX: f32 = 70.0;
pub const BOARD_CELL_HEIGHT_PX_HALF: f32 = BOARD_CELL_HEIGHT_PX / 2.0;

// audio
pub const AUDIO_ERROR_ID: &str = "error";
pub const AUDIO_ERROR_URI: &str = "assets/audio/error.mp3";
pub const AUDIO_JUMP_URI: &str = "assets/audio/jump.mp3";

// UI images
pub const RESET_URI: &str = "/assets/images/return.png";

// death animation
pub const DEATH_DURATION: u32 = 200;
pub const DEATH_ALPHA: f32 = 0.0;
pub const DEATH_SCALE: V2 = V2::ZERO;

use kit::math::*;

// grid
pub const BOARD_SIZE: u32 = 5;
pub const Y_HEX_FACTOR: f32 = 0.8;
pub const BOARD_GRID_SIZE: u32 = 92;
pub const BOARD_ROW_HEIGHT: u32 = BOARD_GRID_SIZE * Y_HEX_FACTOR;

// audio
pub const AUDIO_ERROR_ID: &str = "error";
pub const AUDIO_ERROR_URI: &str = "assets/audio/error.mp3";
pub const AUDIO_JUMP_URI: &str = "assets/audio/jump.mp3";

// UI images
pub const RESET_URI: &str = "/assets/images/return.png";

// pegs
struct PegSheet {
  uri: &str,
  width: u32,
  height: u32,
  length: u32,
}
struct PegProps {
  pivot: V2,
  sheet: PegSheet,
}
pub const PEG_PROPS: PegProps = PegProps {
  pivot: v2(0.5, 0.8),
  sheet: PegSheet {
    uri: "/assets/images/pegs.png",
    width: 40,
    height: 66,
    length: 5,
  },
};

// death animation
pub const DEATH_DURATION: u32 = 200;
pub const DEATH_ALPHA: f32 = 0.0;
pub const DEATH_SCALE: V2 = V2::ZERO;

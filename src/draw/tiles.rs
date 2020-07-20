use crate::board::board_iterator;
use crate::constants::*;
use crate::types::Coords;
use crate::types::State;
use crate::utils::board_to_screen_position;
use kit::geometry::Rect;
use kit::*;

const PIVOT_PERCENT: V2 = v2(0.5, 0.625);
const PIVOT_PX: V2 = v2(
  PIVOT_PERCENT.x * TILE_WIDTH as f32,
  PIVOT_PERCENT.y * TILE_HEIGHT as f32,
);
const TILE_WIDTH: u32 = 92 * 2;
const TILE_HEIGHT: u32 = 126 * 2;
const SCALE: f32 = BOARD_CELL_WIDTH_PX as f32 / TILE_WIDTH as f32;

pub fn draw(ctx: &mut Ctx, state: &State) {
  for pos in board_iterator() {
    draw_tile(ctx, state, pos);
  }
}

fn draw_tile(ctx: &mut Ctx, state: &State, pos: Coords) {
  let screen_pos = board_to_screen_position(pos);
  let texture = state.assets.tile.unwrap();
  let texture_id = texture.id;
  kit::graphics::draw_image(
    ctx,
    texture_id,
    screen_pos,
    SCALE,
    Pivot::Px(PIVOT_PX.x, PIVOT_PX.y),
  );
}

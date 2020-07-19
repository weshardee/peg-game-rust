use crate::board::board_iterator;
use crate::constants::*;
use crate::types::Coords;
use crate::types::State;
use crate::utils::board_to_screen_position;
use kit::geometry::Rect;
use kit::math::*;
use kit::*;

const PIVOT_PERCENT: V2 = v2(0.5, 0.375);
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
  kit::graphics::draw_image(ctx, texture_id, screen_pos, SCALE, PIVOT_PX);
  //
  let half_w = TILE_WIDTH as f32 * SCALE * PIVOT_PERCENT.x;
  let up_h = TILE_HEIGHT as f32 * SCALE * (1.0 - PIVOT_PERCENT.y);
  let down_h = TILE_HEIGHT as f32 * SCALE * PIVOT_PERCENT.y;
  kit::graphics::draw_rect(
    ctx,
    Rect {
      min_x: screen_pos.x - half_w,
      max_x: screen_pos.x + half_w,
      min_y: screen_pos.y - down_h,
      max_y: screen_pos.y + up_h,
    },
    V4::RED,
  );

  let pos_v3: V3 = screen_pos.into();
  let up = pos_v3 + V3::Y * 10.0;
  let down = pos_v3 - V3::Y * 10.0;
  let right = pos_v3 + V3::X * 10.0;
  let left = pos_v3 - V3::X * 10.0;
  kit::graphics::draw_line(ctx, up, down, V4::RED);
  kit::graphics::draw_line(ctx, left, right, V4::RED);

  // TODO animation
  // kit::graphics::draw_sprite()
  // return (
  //   <Motion
  //     defaultStyle={{ y: screenPos.y + 600 }}
  //     style={{ y: spring(screenPos.y) }}
  //     children={({ y }: AnimatedProps) => (
  //       <Image
  //         src={tileSVG}
  //         pivot={{ x: 0.5, y: 0.375 }}
  //         width={TILE_WIDTH}
  //         height={TILE_HEIGHT}
  //         x={screenPos.x}
  //         y={y}
  //         scale={TILE_SCALE}
  //         onClick={() => onTouchTile(props)}
  //       />
  //     )}
  //   />
  // );
}

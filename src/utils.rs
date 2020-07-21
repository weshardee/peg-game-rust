use crate::board::Board;
use crate::constants::*;
use crate::types::Coords;
use crate::types::PegType;
use kit::window_height;
use kit::window_width;
use kit::*;

pub fn middle_position(a: Coords, b: Coords) -> Coords {
  let delta_x = a.x - b.x;
  let delta_y = a.y - b.y;
  return Coords {
    x: delta_x / 2 + b.x,
    y: delta_y / 2 + b.y,
  };
}

pub fn is_valid_move(board: &Board, from: Coords, to: Coords) -> bool {
  if to.x > to.y || to.y >= BOARD_SIZE || to.y < 0 || to.x < 0 {
    return false;
  }
  let middle = middle_position(from, to);
  return board.get(middle).is_some() && board.get(to).is_none();
}

const MOVE_DIRECTIONS: [Coords; 6] = [
  Coords { x: 0, y: 2 },
  Coords { x: 0, y: -2 },
  Coords { x: -2, y: -2 },
  Coords { x: -2, y: 0 },
  Coords { x: 2, y: 2 },
  Coords { x: 2, y: 0 },
];

pub fn has_valid_moves(board: &Board, from: Coords) -> bool {
  // make sure there's a peg at the coordinates
  if board.get(from).is_none() {
    false
  } else {
    for i in 0..6 {
      let to = MOVE_DIRECTIONS[i] + from;
      if is_valid_move(board, from, to) {
        return true;
      }
    }
    return false;
  }
}

/// returns a visual offset for the given row in pixel-space
/// so that drawing the board creates a pyramid shape which
/// is centered on screen
fn board_to_screen_position_offset_x(y: i32) -> f32 {
  let y = y as f32;
  -y * BOARD_CELL_WIDTH_PX_HALF
}

pub fn board_to_screen_position(pos: Coords) -> Vec2 {
  let offset_x = board_to_screen_position_offset_x(pos.y);
  let offset_y = BOARD_SIZE as f32 * BOARD_CELL_HEIGHT_PX_HALF - 60.0;
  vec2(
    pos.x as f32 * BOARD_CELL_WIDTH_PX + offset_x,
    -pos.y as f32 * BOARD_CELL_HEIGHT_PX + offset_y,
  )
}

pub fn screen_to_board_position(screen_pos: Vec2) -> Coords {
  // let y = (screen_pos.y / BOARD_ROW_HEIGHT) as i32;
  // let x = ((screen_pos.x - x_offset(y)) / BOARD_GRID_SIZE as f32) as i32;
  let x = 0;
  let y = 0; // TODO
  return Coords { x, y };
}

pub fn are_equal(a: Coords, b: Coords) -> bool {
  return a.x == b.x && a.y == b.y;
}

pub fn test_peg(ctx: &Ctx, board_pos: Coords, peg_type: PegType, p: Vec2) -> bool {
  let (head, body, feet) = peg_bounds(ctx, board_pos, peg_type);
  test_point_v_circle(p, head) || test_point_v_circle(p, feet) || test_point_v_aabb(p, body)
}

pub fn peg_bounds(ctx: &Ctx, pos: Coords, peg_type: PegType) -> (Circle, Rect, Circle) {
  let r = 36.0;
  let height = match peg_type {
    PegType::Beige => 50.0,
    PegType::Pink => 50.0,
    PegType::Yellow => 40.0,
    PegType::Green => 50.0,
    PegType::Blue => 50.0,
  };
  let base = board_to_screen_position(pos) + vec2(0.0, 10.0);
  let top = base + vec2(0.0, height);
  let center = top;
  // TODO would be nice if we implemented positional add
  // for circle shapes
  let head = Circle { r, center: top };
  let feet = Circle { r, center: base };
  let body = Rect {
    min_x: base.x() - r,
    max_x: base.x() + r,
    min_y: base.y(),
    max_y: top.y(),
  };
  // TODO Capsule hit shape? Hit shape groups?
  (head, body, feet)
}

pub fn window_center_px(ctx: &Ctx) -> Vec2 {
  vec2(window_width(ctx) as f32, window_height(ctx) as f32) / 2.0
}

pub fn window_to_world(ctx: &Ctx, p: Vec2) -> Vec2 {
  let window_width = window_width(ctx) as f32;
  let window_height = window_height(ctx) as f32;
  let world_pos = vec2(p.x() - window_width / 2.0, window_height / 2.0 - p.y());

  // TODO mouse coords are y-down and relative to the upper
  // left corner of the screen - kit's default 2D rendering
  // view uses y-up and the origin is at the center of the
  // window. This makes converting mouse pos to world pos
  // for hit testing a bit hairy (see below for a general
  // approach). Could I do GPU-based hit testing? How can
  // I make hit testing simpler? Should the default coord
  // system use the window coordinates (y-down, upper-left
  // origin)?
  // let world_to_window_vp = ctx.gl.view * ctx.gl.proj;
  // let window_to_world_vp = world_to_window_vp.inverse();
  // let window_normalized_pos = vec4(
  //   lerpf(-1.0, 1.0, p.x() / window_width),
  //   lerpf(1.0, -1.0, p.y() / window_height),
  //   0.0,
  //   1.0,
  // );
  // let world_pos = window_to_world_vp * window_normalized_pos;
  // let world_pos = vec2(world_pos.x(), world_pos.y());

  world_pos
}

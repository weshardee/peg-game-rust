use crate::board::Board;
use crate::constants::*;
use crate::types::Coords;
use kit::math::*;

pub fn get_middle_position(a: Coords, b: Coords) -> Coords {
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
  let middle = get_middle_position(from, to);
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

pub fn board_to_screen_position(pos: Coords) -> V2 {
  let offset_x = board_to_screen_position_offset_x(pos.y);
  let offset_y = BOARD_SIZE as f32 * BOARD_CELL_HEIGHT_PX_HALF - 80.0;
  v2(
    pos.x as f32 * BOARD_CELL_WIDTH_PX + offset_x,
    -pos.y as f32 * BOARD_CELL_HEIGHT_PX + offset_y,
  )
}

pub fn screen_to_board_position(screen_pos: V2) -> Coords {
  // let y = (screen_pos.y / BOARD_ROW_HEIGHT) as i32;
  // let x = ((screen_pos.x - x_offset(y)) / BOARD_GRID_SIZE as f32) as i32;
  let x = 0;
  let y = 0; // TODO
  return Coords { x, y };
}

pub fn are_equal(a: Coords, b: Coords) -> bool {
  return a.x == b.x && a.y == b.y;
}

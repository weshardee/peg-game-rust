use crate::board::board_iterator;
use crate::types::*;
use crate::utils::peg_bounds;
use kit::*;

pub fn draw(ctx: &mut Ctx, state: &State) {
  draw_debug_mouse_pos(ctx, state);
  draw_debug_peg_hit_areas(ctx, state);
}

/// draws a circle around the mouse position in world space
/// to help debug logic that converts window position to
/// world position.
///
/// TODO mouse pos seems to noticably lag
fn draw_debug_mouse_pos(ctx: &mut Ctx, state: &State) {
  let center = state.mouse_pos;
  draw_circ(ctx, Circle { center, r: 10.0 }, red());
}

/// draws hit bounderies for pegs and highlights the "hovered"
/// peg boundary.
fn draw_debug_peg_hit_areas(ctx: &mut Ctx, state: &State) {
  let over_peg = state.over_peg;

  for pos in state.board.iterator() {
    let peg_i = state.board.get(pos);
    let color = if (over_peg.is_some() && over_peg.unwrap() == (pos)) {
      white()
    } else {
      red()
    };
    match peg_i {
      Some(peg_i) => {
        let peg_type = state.pegs.peg_type[peg_i];
        draw_peg_bounds(ctx, pos, peg_type, color);
      }
      None => {}
    }
  }
}

pub fn draw_peg_bounds(ctx: &mut Ctx, pos: Coords, peg_type: PegType, color: Vec4) {
  // TODO maybe store bounds on state to avoid recalc
  let (head, body, feet) = peg_bounds(ctx, pos, peg_type);
  draw_circ(ctx, head, color);
  draw_circ(ctx, feet, color);
  draw_rect(ctx, body, color);
}

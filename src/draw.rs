mod gui;
mod pegs;
mod tiles;

use crate::board::board_iterator;
use crate::types::PegType;
use crate::types::State;
use kit::*;

pub fn draw(ctx: &mut Ctx, state: &State) {
  // TODO rename to something like `project_2d(ctx, world_w, world_h)`
  kit::graphics::default_projection(ctx);
  tiles::draw(ctx, state);
  pegs::draw(ctx, state);
  gui::draw(ctx, state);
}

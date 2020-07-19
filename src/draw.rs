mod tiles;

use crate::board::board_iterator;
use crate::types::PegType;
use crate::types::State;
use kit::math::*;
use kit::*;

pub fn draw(ctx: &mut Ctx, state: &State) {
  kit::graphics::default_projection(ctx);
  tiles::draw(ctx, state);
}

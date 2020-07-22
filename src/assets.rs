use crate::types::*;
use kit::*;

pub fn init(ctx: &mut Ctx, state: &mut State) {
  state.assets.tile = Some(load_img(ctx, "assets/images/tile.png"));
  state.assets.shadow = Some(load_img(ctx, "assets/images/shadow.png"));
  state.assets.peg_beige = Some(load_img(ctx, "assets/images/peg_beige.png"));
  state.assets.peg_blue = Some(load_img(ctx, "assets/images/peg_blue.png"));
  state.assets.peg_green = Some(load_img(ctx, "assets/images/peg_green.png"));
  state.assets.peg_pink = Some(load_img(ctx, "assets/images/peg_pink.png"));
  state.assets.peg_yellow = Some(load_img(ctx, "assets/images/peg_yellow.png"));
}

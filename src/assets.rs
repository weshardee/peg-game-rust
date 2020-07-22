use crate::types::*;
use kit::*;

pub fn init(ctx: &mut Ctx, state: &mut State) {
  state.assets.tile = Some(kit::load_img(ctx, "assets/images/tile.png"));
  state.assets.shadow = Some(kit::load_img(ctx, "assets/images/shadow.png"));
  state.assets.peg_beige = Some(kit::load_img(ctx, "assets/images/peg_beige.png"));
  state.assets.peg_blue = Some(kit::load_img(ctx, "assets/images/peg_blue.png"));
  state.assets.peg_green = Some(kit::load_img(ctx, "assets/images/peg_green.png"));
  state.assets.peg_pink = Some(kit::load_img(ctx, "assets/images/peg_pink.png"));
  state.assets.peg_yellow = Some(kit::load_img(ctx, "assets/images/peg_yellow.png"));
}

use crate::types::*;
use kit::graphics::sprite;
use kit::*;

pub fn init(ctx: &mut Ctx, state: &mut State) {
  let id = state.assets.peg_pink.unwrap().id;
  state.sprites.peg_pink = PegSheet {
    jump: sprite(ctx, id, 1, 1, 67, 93, Pivot::Px(30.0, 0.0)),
    hurt: sprite(ctx, id, 1, 96, 69, 93, Pivot::Px(45.0, 0.0)),
    front: sprite(ctx, id, 1, 190, 66, 92, Pivot::Px(32.0, 0.0)),
    lean: sprite(ctx, id, 1, 284, 66, 92, Pivot::Px(28.0, 0.0)),
    duck: sprite(ctx, id, 1, 378, 69, 71, Pivot::Px(28.0, 0.0)),
  };
}

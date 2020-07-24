use crate::types::State;
use kit::*;

// TODO load reset icon asset "images/reset.png";

fn reset_btn() {
  // TODO interactions
  // TODO spin on click
}

fn status_msg(ctx: &mut Ctx, state: &State) {
  // TODO show on game over
  // TODO fade in
}

pub fn draw(ctx: &mut Ctx, state: &State) {
  // TODO  GUI (reset_btn & status_msg)
  status_msg(ctx, state);
  // TODO mute button
}

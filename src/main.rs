#![allow(dead_code)]
#![allow(unused)]

mod board;
mod constants;
mod draw;
mod sprites;
mod types;
mod utils;

use crate::board::board_iterator;
use crate::constants::BOARD_SIZE;
use crate::types::*;
use kit::*;
use rand;
use std::time::Duration;
use utils::*;

const TITLE: &str = "Peg Game";

const DT_NANOS: u64 = 1_000_000_000 / 60;
const DT: Duration = Duration::from_nanos(DT_NANOS);

#[derive(Default)]
struct Assets {}

struct App {
  state: State,
}

impl KApp for App {
  fn new() -> Self {
    Self {
      state: Default::default(),
    }
  }

  fn init(&mut self, ctx: &mut Ctx) {
    let state = &mut self.state;

    // load assets
    state.assets.tile = Some(kit::load_img(ctx, "assets/images/tile.png"));
    state.assets.shadow = Some(kit::load_img(ctx, "assets/images/shadow.png"));
    state.assets.peg_beige = Some(kit::load_img(ctx, "assets/images/peg_beige.png"));
    state.assets.peg_blue = Some(kit::load_img(ctx, "assets/images/peg_blue.png"));
    state.assets.peg_green = Some(kit::load_img(ctx, "assets/images/peg_green.png"));
    state.assets.peg_pink = Some(kit::load_img(ctx, "assets/images/peg_pink.png"));
    state.assets.peg_yellow = Some(kit::load_img(ctx, "assets/images/peg_yellow.png"));

    sprites::init(ctx, state);

    // set initial pegs on board
    let mut peg_i = 0;
    for pos in board_iterator() {
      if pos.x == 1 && pos.y == 2 {
        continue;
      }
      let peg_type: PegType = rand::random();
      state.pegs.peg_type[peg_i] = peg_type;
      state.board.set(pos, Some(peg_i));
      peg_i += 1;
    }
  }

  fn frame(&mut self, ctx: &mut Ctx) {
    let state = &mut self.state;

    // TODO gamepad input

    process_mouse_pos(ctx, state);
    process_over_peg(ctx, state);
    process_phase(ctx, state);

    // TODO animate peg hover state
    // TODO animate peg selected state
    // TODO animate peg buzz state

    draw::draw(ctx, state);
  }
}

fn main() {
  run::<App>(KAppDesc {
    window_title: TITLE.to_string(),
    ..Default::default()
  });
}

/// calculates the mouse's position in world in order to
fn process_mouse_pos(ctx: &Ctx, state: &mut State) {
  let mouse_pos = ctx.input.mouse.pos;
  state.mouse_pos = window_to_world(ctx, mouse_pos);
}

fn process_over_peg(ctx: &Ctx, state: &mut State) {
  let mouse_pos = state.mouse_pos;
  state.over_peg = None;
  for pos in board_iterator() {
    let peg_i = state.board.get(pos);
    match peg_i {
      Some(peg_i) => {
        let peg_type = state.pegs.peg_type[peg_i];
        if test_peg(ctx, pos, peg_type, mouse_pos) {
          state.over_peg = Some(pos);
        }
      }
      None => {}
    }
  }
}

fn process_phase(ctx: &Ctx, state: &mut State) {
  match state.phase {
    Phase::Ready => {
      // TODO
    }
    Phase::Picking => process_phase_picking(ctx, state),
    Phase::Excited(pos) => {
      // TODO
    }
  }
}

fn process_phase_picking(ctx: &Ctx, state: &mut State) {
  // TODO clicks aren't always registering, but only when not plugged in? Debug this!
  if ctx.input.mouse.left.down > 0 {
    let picked_peg = state.over_peg;
    match picked_peg {
      None => {}
      Some(pos) => {
        let peg_i = state.board.get(pos).unwrap(); // picked position must have a peg!
        let valid = has_valid_moves(&state.board, pos);
        if valid {
          state.phase = Phase::Excited(pos);
        } else {
          state.pegs.state[peg_i] = PegState::Hurt;
          // TODO buzz peg with no moves
          // TODO audio
        }
      }
    }
  }
}

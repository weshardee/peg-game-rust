#![allow(dead_code)]
#![allow(unused)]

mod assets;
mod board;
mod constants;
mod draw;
mod sprites;
mod types;
mod utils;

use crate::board::board_iterator;
use crate::constants::BOARD_SIZE;
use crate::constants::MAX_PEGS;
use crate::types::*;
use constants::*;
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

    assets::init(ctx, state);
    sprites::init(ctx, state);

    // set initial pegs on board
    let empty = Coords { x: 1, y: 2 };
    populate(state, empty);
  }

  fn frame(&mut self, ctx: &mut Ctx) {
    let state = &mut self.state;

    // TODO gamepad input

    process_mouse_pos(ctx, state);
    process_over_peg(ctx, state);
    process_peg_animations(ctx, state);
    process_peg_z(ctx, state);
    process_phase(ctx, state);

    // TODO animate peg hover state

    draw::draw(ctx, state);
  }
}

fn main() {
  run::<App>(KAppDesc {
    window_title: TITLE.to_string(),
    ..Default::default()
  });
}

/// fills the game board with pegs; only call when the board is empty!
fn populate(state: &mut State, empty: Coords) {
  let mut peg_i = 0;
  for pos in board_iterator() {
    if pos.x == 1 && pos.y == 2 {
      continue;
    }
    let peg_type: PegType = rand::random();
    state.pegs.peg_type[peg_i] = peg_type;
    state.pegs.z[peg_i] = rand::random::<f32>() * DROP_HEIGHT_VARIANCE + DROP_HEIGHT_MIN;
    state.board.set(pos, Some(peg_i));
    peg_i += 1;
  }
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
  let click = ctx.input.mouse.left.down > 0;
  if click {
    let picked_peg = state.over_peg;
    match picked_peg {
      None => {}
      Some(pos) => {
        let peg_i = state.board.get(pos).unwrap(); // picked position must have a peg!
        let valid = has_valid_moves(&state.board, pos);
        if valid {
          state.phase = Phase::Excited(pos);
          state.pegs.state[peg_i] = PegState::Excited;
          state.pegs.animation[peg_i] = 0;
        } else {
          state.pegs.state[peg_i] = PegState::Buzz;
          state.pegs.animation[peg_i] = 0;
          // TODO audio
        }
      }
    }
  }
}

fn process_peg_z(ctx: &Ctx, state: &mut State) {
  for i in 0..MAX_PEGS {
    let z = state.pegs.z[i];
    let z_vel = state.pegs.z_vel[i];
    let z_vel = z_vel + DROP_GRAVITY_PER_DT; // add in acceleration
    let mut z_vel = clampf(z_vel, -DROP_TERMINAL_VEL, DROP_TERMINAL_VEL);
    let mut z = z + z_vel;

    if z.abs() < 1.0 && z_vel.abs() < 1.0 {
      // gotta stop sometime
      z = 0.0;
      z_vel = 0.0;
    } else if z < 0.0 {
      // can't be below ground! flip to bounce
      z = -z;
      z_vel = -z_vel * DROP_BOUNCE_DAMPENING; // flip and dampen vel
    }
    state.pegs.z[i] = z;
    state.pegs.z_vel[i] = z_vel;
  }
}

fn process_peg_animations(ctx: &Ctx, state: &mut State) {
  for i in 0..MAX_PEGS {
    state.pegs.animation[i] += 1;
  }

  for i in 0..MAX_PEGS {
    let animation = state.pegs.animation[i];
    let peg_state = state.pegs.state[i];
    match (peg_state) {
      (PegState::Buzz) => {
        if (animation > BUZZ_STATE_DURATION) {
          state.pegs.state[i] = PegState::Front;
          state.pegs.animation[i] = 0;
        }
      }
      _ => {}
    }
  }
}

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
    state.assets.tile = Some(kit::graphics::load_img(ctx, "assets/images/tile.png"));
    state.assets.shadow = Some(kit::graphics::load_img(ctx, "assets/images/shadow.png"));
    state.assets.peg_beige = Some(kit::graphics::load_img(ctx, "assets/images/peg_beige.png"));
    state.assets.peg_blue = Some(kit::graphics::load_img(ctx, "assets/images/peg_blue.png"));
    state.assets.peg_green = Some(kit::graphics::load_img(ctx, "assets/images/peg_green.png"));
    state.assets.peg_pink = Some(kit::graphics::load_img(ctx, "assets/images/peg_pink.png"));
    state.assets.peg_yellow = Some(kit::graphics::load_img(ctx, "assets/images/peg_yellow.png"));

    sprites::init(ctx, state);

    // set initial pegs on board
    for pos in board_iterator() {
      if pos.x == 1 && pos.y == 2 {
        continue;
      }
      let peg_type: PegType = rand::random();
      state.board.set(pos, Some(peg_type));
    }
  }

  fn frame(&mut self, ctx: &mut Ctx) {
    let state = &mut self.state;
    draw::draw(ctx, state);
  }
}

fn main() {
  run::<App>(KAppDesc {
    window_title: TITLE.to_string(),
    ..Default::default()
  });
}

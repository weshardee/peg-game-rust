#![allow(dead_code)]
#![allow(unused)]

mod board;
mod constants;
mod draw;
mod types;
mod utils;

use crate::board::board_iterator;
use crate::constants::BOARD_SIZE;
use crate::types::*;
use kit::*;
use rand;

const TITLE: &str = "Peg Game";

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

    state.assets.tile = Some(kit::graphics::load_img(ctx, "assets/images/tile.png"));

    for pos in board_iterator() {
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

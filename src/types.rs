use crate::board::Board;
use kit::math::*;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use std::collections::HashMap;
use std::ops::Add;

pub type EntityID = String;

// pub enum Action {
//   Touch,
//   Wipe,
//   Populate(Vec<Peg>),
//   Excite(String),
//   Buzz(String),
//   Move {
//     id: String,
//     from: Coords,
//     to: Coords,
//     kill: Peg,
//   },
// }

pub enum PegState {
  Front,
  Lean,
  Jump,
  Duck,
  Hurt,
}

#[derive(Copy, Clone)]
pub enum PegType {
  Beige,
  Blue,
  Green,
  Pink,
  Yellow,
}

impl Distribution<PegType> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PegType {
    match rng.gen_range(0, 5) {
      0 => PegType::Beige,
      1 => PegType::Blue,
      2 => PegType::Green,
      3 => PegType::Pink,
      4 => PegType::Yellow,
      _ => panic!("not a valid random peg type!"),
    }
  }
}

pub enum Phase {
  Ready,
  Picking,
  Excited,
}

#[derive(Copy, Clone, Default)]
pub struct Coords {
  pub x: i32,
  pub y: i32,
}

impl Into<V2> for Coords {
  fn into(self) -> V2 {
    v2(self.x as f32, self.y as f32)
  }
}

impl Add for Coords {
  type Output = Self;
  fn add(self, r: Coords) -> Self {
    Coords {
      x: self.x + r.y,
      y: self.y + r.y,
    }
  }
}

pub struct Peg {
  pub id: String,
  pub pos: Coords,
  pub peg_type: PegType,
}

// pub struct SimpleSpritesheet {
//   : str,
//   width: usize,
//   height: usize,
// }

pub struct PegFrameSize {
  pub w: usize,
  pub h: usize,
}

pub struct PegFrame {
  // src: str,
  size: PegFrameSize,
  // pivot: V2
}

#[derive(Default)]
pub struct Assets {
  pub tile: Option<kit::Texture>,
}

#[derive(Default)]
pub struct State {
  pub board: Board,
  pub assets: Assets,
}

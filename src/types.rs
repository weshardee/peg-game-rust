use crate::board::Board;
use kit::*;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use std::collections::HashMap;
use std::ops::Add;
use std::time::Duration;

pub type EntityID = String;

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

impl Default for Phase {
  fn default() -> Self {
    // TODO should be ready, but starting in Picking for testing purposes
    Phase::Picking
  }
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

pub enum PegFrame {
  Duck,
  Front,
  Hurt,
  Jump,
  Lean,
}

impl Default for PegFrame {
  fn default() -> Self {
    PegFrame::Front
  }
}

#[derive(Default)]
pub struct Assets {
  pub tile: Option<Texture>,
  pub shadow: Option<Texture>,
  pub peg_beige: Option<Texture>,
  pub peg_blue: Option<Texture>,
  pub peg_green: Option<Texture>,
  pub peg_pink: Option<Texture>,
  pub peg_yellow: Option<Texture>,
}

#[derive(Default)]
pub struct PegSheet {
  pub duck: Sprite,
  pub front: Sprite,
  pub hurt: Sprite,
  pub jump: Sprite,
  pub lean: Sprite,
}

#[derive(Default)]
pub struct Sprites {
  pub peg_beige: PegSheet,
  pub peg_blue: PegSheet,
  pub peg_green: PegSheet,
  pub peg_pink: PegSheet,
  pub peg_yellow: PegSheet,
}

#[derive(Default)]
pub struct State {
  pub board: Board,
  pub assets: Assets,
  pub sprites: Sprites,
  pub phase: Phase,
}

use crate::board::Board;
use crate::constants::MAX_PEGS;
use kit::*;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::Rng;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Add;
use std::time::Duration;

pub type EntityID = String;

#[derive(Copy, Clone)]
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
  Excited(Coords),
}

impl Default for Phase {
  fn default() -> Self {
    // TODO should be ready, but starting in Picking for testing purposes
    Phase::Picking
  }
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Coords {
  pub x: i32,
  pub y: i32,
}

impl Display for Coords {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "Coords({}, {})", self.x, self.y)
  }
}

impl Into<Vec2> for Coords {
  fn into(self) -> Vec2 {
    vec2(self.x as f32, self.y as f32)
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

pub struct Pegs {
  pub peg_type: [PegType; MAX_PEGS],
  pub state: [PegState; MAX_PEGS],
}

impl Default for Pegs {
  fn default() -> Self {
    Self {
      peg_type: [PegType::Beige; MAX_PEGS],
      state: [PegState::Front; MAX_PEGS],
    }
  }
}

#[derive(Default)]
pub struct State {
  pub board: Board,             // mapping of board positions to peg_i
  pub assets: Assets,           // loaded assets for rendering
  pub sprites: Sprites,         // asset metadata for rendering
  pub phase: Phase,             // major game phase
  pub mouse_pos: Vec2,          // mouse position in world space
  pub over_peg: Option<Coords>, // peg coords if mouse is over one
  pub pegs: Pegs,               // peg properties types by peg_i
}

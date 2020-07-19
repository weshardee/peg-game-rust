mod beige;
mod blue;
mod green;
mod pink;
mod yellow;

use crate::types::PegType;
use kit::math::V2;
use rand::{
  distributions::{Distribution, Standard},
  Rng,
};
use std::collections::HashMap;


pub type Sheet = HashMap<PegState, PegFrame>;

pub type StateSheet = HashMap<PegType, Sheet>;

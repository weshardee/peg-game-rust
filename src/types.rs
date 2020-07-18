
use type { PegType, PegState } from 'crate::sprites/peg/peg';
pub type { PegType, PegState };

pub type EntityID = string;
pub type Coords = { x: f32, y: f32 };

pub type Peg = {
  id: string,
  pos: Coords,
  type: PegType /* indicates which sprite gets used */,
};

pub type SimpleSpritesheet = {
  uri: string,
  width: f32,
  height: f32,
};

use beige from 'crate::beige';
use blue from 'crate::blue';
use green from 'crate::green';
use pink from 'crate::pink';
use yellow from 'crate::yellow';

use preload from '../../preload';

pub type Frame = {
  src: string,
  size: { w: f32, h: f32 },
  pivot: { x: f32, y: f32 },
};

pub type PegState = 'front' | 'lean' | 'jump' | 'duck' | 'hurt';

pub type Sheet = { [key: PegState]: Frame };

const sprites = { beige, yellow, blue, pink, green };

pub type PegType = $Keys<typeof sprites>;
pub type StateSheet = { [key: PegType]: Sheet };
pub default sprites;

// preload sources
for (let color in sprites) {
  if (sprites.hasOwnProperty(color)) {
    for (let state in sprites[color]) {
      if (sprites[color].hasOwnProperty(state)) {
        const { src } = sprites[color][state];
        preload(src);
      }
    }
  }
}

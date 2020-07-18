use duckSVG from 'crate::pink/duck.svg';
use frontSVG from 'crate::pink/front.svg';
use jumpSVG from 'crate::pink/jump.svg';
use leanSVG from 'crate::pink/lean.svg';
use hurtSVG from 'crate::pink/hurt.svg';

use type { Sheet } from 'crate::types';

const pink: Sheet = {
  duck: {
    src: duckSVG,
    size: { w: 69, h: 71 },
    pivot: { x: 0.405797, y: 1 },
  },
  front: {
    src: frontSVG,
    size: { w: 66, h: 92 },
    pivot: { x: 0.484848, y: 1 },
  },
  hurt: {
    src: hurtSVG,
    size: { w: 69, h: 92 },
    pivot: { x: 0.652174, y: 1 },
  },
  jump: {
    src: jumpSVG,
    size: { w: 67, h: 93 },
    pivot: { x: 0.447761, y: 1 },
  },
  lean: {
    src: leanSVG,
    size: { w: 66, h: 92 },
    pivot: { x: 0.424242, y: 1 },
  },
};

pub default pink;

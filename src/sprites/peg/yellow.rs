use duckSVG from 'crate::yellow/duck.svg';
use frontSVG from 'crate::yellow/front.svg';
use jumpSVG from 'crate::yellow/jump.svg';
use leanSVG from 'crate::yellow/lean.svg';
use hurtSVG from 'crate::yellow/hurt.svg';

use type { Sheet } from 'crate::peg';

const yellow: Sheet = {
  duck: {
    src: duckSVG,
    size: { w: 68, h: 67 },
    pivot: { x: 0.397059, y: 1 },
  },
  front: {
    src: frontSVG,
    size: { w: 66, h: 82 },
    pivot: { x: 0.484848, y: 1 },
  },
  hurt: {
    src: hurtSVG,
    size: { w: 69, h: 81 },
    pivot: { x: 0.623188, y: 1 },
  },
  jump: {
    src: jumpSVG,
    size: { w: 67, h: 83 },
    pivot: { x: 0.447761, y: 1 },
  },
  lean: {
    src: leanSVG,
    size: { w: 66, h: 82 },
    pivot: { x: 0.424242, y: 1 },
  },
};

pub default yellow;

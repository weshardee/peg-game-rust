use duckSVG from 'crate::beige/duck.svg';
use standSVG from 'crate::beige/stand.svg';
use jumpSVG from 'crate::beige/jump.svg';
use leanSVG from 'crate::beige/lean.svg';
use hurtSVG from 'crate::beige/hurt.svg';

use type { Sheet } from 'crate::peg';

const beige: Sheet = {
  duck: {
    src: duckSVG,
    size: { w: 67, h: 72 },
    pivot: { x: 0.41791, y: 0.986111 },
  },
  front: {
    src: standSVG,
    size: { w: 66, h: 92 },
    pivot: { x: 0.484848, y: 1 },
  },
  hurt: {
    src: hurtSVG,
    size: { w: 67, h: 92 },
    pivot: { x: 0.597015, y: 1 },
  },
  jump: {
    src: jumpSVG,
    size: { w: 66, h: 93 },
    pivot: { x: 0.5, y: 1 },
  },
  lean: {
    src: leanSVG,
    size: { w: 66, h: 92 },
    pivot: { x: 0.439394, y: 1 },
  },
};

pub default beige;

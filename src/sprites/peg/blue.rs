use duckSVG from 'crate::blue/duck.svg';
use frontSVG from 'crate::blue/front.svg';
use jumpSVG from 'crate::blue/jump.svg';
use leanSVG from 'crate::blue/lean.svg';
use hurtSVG from 'crate::blue/hurt.svg';

use type { Sheet } from 'crate::peg';

const blue: Sheet = {
  duck: {
    src: duckSVG,
    size: { w: 67, h: 72 },
    pivot: { x: 0.41791, y: 0.986111 },
  },
  front: {
    src: frontSVG,
    size: { w: 66, h: 92 },
    pivot: { x: 0.484848, y: 1 },
  },
  hurt: {
    src: hurtSVG,
    size: { w: 67, h: 92 },
    pivot: { x: 0.61194, y: 1 },
  },
  jump: {
    src: jumpSVG,
    size: { w: 66, h: 93 },
    pivot: { x: 0.469697, y: 1 },
  },
  lean: {
    src: leanSVG,
    size: { w: 66, h: 92 },
    pivot: { x: 0.439394, y: 1 },
  },
};

pub default blue;

const DUCK_SVG: &str = "blue/duck.svg";
const FRONT_SVG: &str = "blue/front.svg";
const JUMP_SVG: &str = "blue/jump.svg";
const LEAN_SVG: &str = "blue/lean.svg";
const HURT_SVG: &str = "blue/hurt.svg";

use crate::peg::PegSheet;

pub const blue: PegSheet = PegSheet {
  duck: PegFrame {
    src: duck_SVG,
    size: PegFrameSize { w: 67, h: 72 },
    pivot: v2(0.41791, 0.986111),
  },
  front: PegFrame {
    src: front_SVG,
    size: PegFrameSize { w: 66, h: 92 },
    pivot: v2(0.484848, 1.0),
  },
  hurt: PegFrame {
    src: hurt_SVG,
    size: PegFrameSize { w: 67, h: 92 },
    pivot: v2(0.61194, 1.0),
  },
  jump: PegFrame {
    src: jump_SVG,
    size: PegFrameSize { w: 66, h: 93 },
    pivot: v2(0.469697, 1.0),
  },
  lean: PegFrame {
    src: lean_SVG,
    size: PegFrameSize { w: 66, h: 92 },
    pivot: v2(0.439394, 1.0),
  },
};

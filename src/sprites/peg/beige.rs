const DUCK_SVG: &str = include_bytes!("beige/duck.svg");
const STAND_SVG: &str = include_bytes!("beige/stand.svg");
const JUMP_SVG: &str = include_bytes!("beige/jump.svg");
const LEAN_SVG: &str = include_bytes!("beige/lean.svg");
const HURT_SVG: &str = include_bytes!("beige/hurt.svg");

use crate::peg::PegSheet;

const BEIGE: PegSheet = PegSheet {
  duck: PegFrame {
    src: DUCK_SVG,
    size: PegFrameSize { w: 67, h: 72 },
    pivot: v2(0.41791, 0.986111),
  },
  front: PegFrame {
    src: STAND_SVG,
    size: PegFrameSize { w: 66, h: 92 },
    pivot: v2(0.484848, 1.0),
  },
  hurt: PegFrame {
    src: HURT_SVG,
    size: PegFrameSize { w: 67, h: 92 },
    pivot: v2(0.597015, 1.0),
  },
  jump: PegFrame {
    src: JUMP_SVG,
    size: PegFrameSize { w: 66, h: 93 },
    pivot: v2(0.5, 1.0),
  },
  lean: PegFrame {
    src: LEAN_SVG,
    size: PegFrameSize { w: 66, h: 92 },
    pivot: v2(0.439394, 1.0),
  },
};

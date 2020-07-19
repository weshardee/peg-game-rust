const DUCK_SVG: &str = "yellow/duck.svg";
const FRONT_SVG: &str = "yellow/front.svg";
const JUMP_SVG: &str = "yellow/jump.svg";
const LEAN_SVG: &str = "yellow/lean.svg";
const HURT_SVG: &str = "yellow/hurt.svg";

use crate::sprites::peg::PegSheet;

const YELLOW: PegSheet = PegSheet {
  duck: PegFrame {
    src: duck_SVG,
    size: PegFrameSize { w: 68, h: 67 },
    pivot: v2(0.397059, 1.0),
  },
  front: PegFrame {
    src: front_SVG,
    size: PegFrameSize { w: 66, h: 82 },
    pivot: v2(0.484848, 1.0),
  },
  hurt: PegFrame {
    src: hurt_SVG,
    size: PegFrameSize { w: 69, h: 81 },
    pivot: v2(0.623188, 1.0),
  },
  jump: PegFrame {
    src: jump_SVG,
    size: PegFrameSize { w: 67, h: 83 },
    pivot: v2(0.447761, 1.0),
  },
  lean: PegFrame {
    src: lean_SVG,
    size: PegFrameSize { w: 66, h: 82 },
    pivot: v2(0.424242, 1.0),
  },
};

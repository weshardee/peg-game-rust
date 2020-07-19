const DUCK_SVG: &str = "pink/duck.svg";
const FRONT_SVG: &str = "pink/front.svg";
const JUMP_SVG: &str = "pink/jump.svg";
const LEAN_SVG: &str = "pink/lean.svg";
const HURT_SVG: &str = "pink/hurt.svg";

use crate::sprites::peg::PegSheet;

pub const PINK: PegSheet = PegSheet {
  duck: PegFrame {
    src: duck_SVG,
    size: PegFrameSize { w: 69, h: 71 },
    pivot: v2(0.405797, 1.0),
  },
  front: PegFrame {
    src: front_SVG,
    size: PegFrameSize { w: 66, h: 92 },
    pivot: v2(0.484848, 1.0),
  },
  hurt: PegFrame {
    src: hurt_SVG,
    size: PegFrameSize { w: 69, h: 92 },
    pivot: v2(0.652174, 1.0),
  },
  jump: PegFrame {
    src: jump_SVG,
    size: PegFrameSize { w: 67, h: 93 },
    pivot: v2(0.447761, 1.0),
  },
  lean: PegFrame {
    src: lean_SVG,
    size: PegFrameSize { w: 66, h: 92 },
    pivot: v2(0.424242, 1.0),
  },
};

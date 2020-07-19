const DUCK_SVG: &str = "green/duck.svg";
const FRONT_SVG: &str = "green/front.svg";
const JUMP_SVG: &str = "green/jump.svg";
const LEAN_SVG: &str = "green/lean.svg";
const HURT_SVG: &str = "green/hurt.svg";

use crate::sprites::peg::PegSheet;

pub const GREEN: PegSheet = PegSheet {
  duck: PegFrame {
    src: duck_SVG,
    size: Size { w: 69, h: 71 },
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
    pivot: v2(0.666667, 1.0),
  },
  jump: PegFrame {
    src: jump_SVG,
    size: PegFrameSize { w: 67, h: 93 },
    pivot: v2(0.432836, 1.0),
  },
  lean: PegFrame {
    src: lean_SVG,
    size: PegFrameSize { w: 66, h: 92 },
    pivot: v2(0.409091, 1.0),
  },
};

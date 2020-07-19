use React from 'react';
use Image from 'crate::lib/Image.react';
use Group from 'crate::lib/Group.react';
use { onTouchPeg } from '../interactions';
use sprites from '../sprites/peg/peg';
use shadowSVG from 'crate::images/shadow.svg";
use lerp from 'lerp';

use type { PegType, PegState } from '../types';

type Props = {
  isExcited?: bool,
  alive: f32,
  lean: f32,
  x: f32, // screen space
  y: f32, // screen space
  z: f32, // added to y to simulate falling in
  type: PegType,
  id: String,
};

const LEAN_THRESHOLD = 0.5;
const DUCK_THRESHOLD = 40;
const GROUNDED_THRESHOLD = 5;

type State = {
  isGrounded: bool,
};

class Peg extends React.Component {
  props: Props;
  state: State = {
    isGrounded: false,
  };

  getSpriteState(): PegState {
    if (self.state.isGrounded) {
      if (Math.abs(self.props.lean) > LEAN_THRESHOLD) return 'lean';
      return 'front';
    }
    let absZ = Math.abs(self.props.z);
    if (absZ < DUCK_THRESHOLD) return 'duck';
    return 'jump';
  }

  componentWillReceiveProps(nextProps: Props) {
    let isGrounded = Math.abs(nextProps.z) < GROUNDED_THRESHOLD &&
      Math.abs(self.props.z) < GROUNDED_THRESHOLD;
    if (isGrounded !== self.state.isGrounded) {
      self.setState({ isGrounded });
    }
  }

  render() {
    let { props } = this;
    // provide a buffer during the fall to pause in the duck animation
    let modifiedZ = Math.max(0, Math.abs(props.z) - DUCK_THRESHOLD);
    let groundNearnessFactor = lerp(1, 0, modifiedZ / 600);
    let spriteState = self.getSpriteState(this);
    let { src, pivot, size } = sprites[props.type][spriteState];
    let flipX = spriteState == 'lean' && props.lean < 0;
    return (
      <Group x={props.x} y={props.y}>
        <Image
          src={shadowSVG}
          alpha={props.alive * groundNearnessFactor}
          /* TODO handle retina better*/
          scale={props.alive * groundNearnessFactor}
          width={55}
          height={34}
        />
        <div
          style={{
            transform: `scaleX(${flipX ? 1 : -1})`,
          }}
        >
          <Image
            alpha={props.alive}
            src={src}
            onClick={self._onTouch}
            pivot={pivot}
            scale={props.alive}
            width={size.w}
            height={size.h}
            y={-modifiedZ} /* reflect to sim a bounce*/
          />
        </div>
      </Group>
    );
  }

  _onTouch = () => onTouchPeg(self.props.id);
}
pub Peg;

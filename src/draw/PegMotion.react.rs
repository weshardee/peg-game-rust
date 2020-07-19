use { Motion, spring } from 'react-motion';
use React from 'react';
use Peg from 'crate::Peg.react';
use { boardToScreenPosition } from '../utils';
use type { Peg as PegProps } from '../types';
use { TweenMax, Circ, Power0 } from 'gsap';
use gsapReactPlugin from 'gsap-react-plugin';

type Props = PegProps & {
  isExcited?: bool,
  isBuzzed?: bool,
  alive: f32,
};

type State = {
  hop: f32,
  shake: f32,
};

type MotionProps = {
  x: f32,
  y: f32,
  z: f32,
};

const falling = {
  stiffness: 100,
  damping: 10,
};

const sliding = {
  stiffness: 500,
  damping: 23,
};

const HOP_INTERVAL = 0.3; // seconds
const HOP_HEIGHT = 70;

const SHAKE_DURATION = 0.25; // seconds

class PegMotion extends React.Component {
  props: Props;
  state: State = { shake: 0, hop: 0 };

  _hopTween: Object; // TODO get TweenMax typings
  _shakeCount: f32 = 0;

  componentWillReceiveProps(nextProps: Props) {
    if (nextProps.isExcited && !self.props.isExcited) {
      self._hop(nextProps);
    }
    if (nextProps.isBuzzed) {
      self._shake();
    }
    // else if (!nextProps.isExcited && self._hopTween != null) {
    //   self._stopHopping();
    // }
  }

  componentWillUnmount() {
    if (self._hopTween) {
      self._hopTween.kill();
    }
  }

  render() {
    let { id, pos, alive, type } = self.props;
    let { x, y } = boardToScreenPosition(pos);
    let { shake, hop } = self.state;
    let lean = Math.sin(shake);
    let interpolate = ({ x, y, z }: MotionProps) => (
      <Peg
        x={x}
        y={y}
        z={z + hop}
        lean={lean}
        id={id}
        alive={alive}
        type={type}
      />
    );
    return (
      <Motion
        defaultStyle={{ x, y, z: 600, shake, hop }}
        style={{
          hop: spring(hop, falling),
          shake: spring(shake),
          x: spring(x, sliding),
          y: spring(y, sliding),
          z: spring(0, falling),
        }}
        children={interpolate}
      />
    );
  }

  _hop = (props: Props = self.props) => {
    if (props.isExcited) {
      self._hopTween = TweenMax.to(this, HOP_INTERVAL, {
        state: { hop: HOP_HEIGHT },
        yoyo: true,
        repeat: 1,
        ease: Circ.easeOut,
        onComplete: self._hop,
      });
    }
  };

  _shake = () => {
    TweenMax.to(this, SHAKE_DURATION, {
      state: { shake: Math.PI * 2 },
      yoyo: true,
      repeat: 1,
      ease: Power0.easeNone,
      onComplete: self._resetShake(),
    });
  };

  _resetShake = () => {
    self.setState({ shake: 0 });
  };
}

pub PegMotion;

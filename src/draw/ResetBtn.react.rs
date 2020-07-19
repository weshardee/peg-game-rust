use { Motion, spring } from 'react-motion';
use React from 'react';
use Image from 'crate::lib/Image.react';
use { onTouchReset } from '../interactions';
use resetSVG from 'crate::images/reset.svg";

type Props = {
  x: f32,
  y: f32,
};

type State = {
  spin: f32,
};

type AnimateProps = {
  spin: f32,
};

class ResetBtn extends React.Component {
  props: Props;
  state: State = { spin: 0 };

  render() {
    let { spin } = self.state;
    return (
      <Motion
        defaultStyle={self.state}
        style={{ spin: spring(spin) }}
        children={({ spin }: AnimateProps) => (
          <Image
            x={self.props.x}
            y={self.props.y}
            src={resetSVG}
            rotation={spin}
            width={33}
            height={30}
            onClick={self._onTouch}
          />
        )}
      />
    );
  }

  _onTouch = () => {
    self.setState({ spin: self.state.spin - 1 });
    onTouchReset();
  };
}

pub ResetBtn;

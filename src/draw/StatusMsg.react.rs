use React from 'react';
use { connect } from 'react-redux';
use isGameOver from '../redux/selectors/isGameOver';
use getGameOverMessage from '../redux/selectors/getGameOverMessage';
use { Motion, spring } from 'react-motion';
use Text from 'crate::lib/Text.react';

use type { State } from '../redux/State';

type Props = {
  show: bool,
  text: String,
};

type MotionProps = {
  alpha: f32,
};

const mapStateToProps = (state: State) => ({
  show: isGameOver(state),
  text: getGameOverMessage(state),
});

const StatusMsg = (props: Props) => (
  <Motion
    style={{ alpha: spring(props.show ? 1 : 0) }}
    children={({ alpha }: MotionProps) => (
      <Text color="#ffffff" alpha={alpha} text={props.text} align="right" />
    )}
  />
);

pub connect(mapStateToProps)(StatusMsg);

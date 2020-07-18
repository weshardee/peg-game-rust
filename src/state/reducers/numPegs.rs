use type { Action } from '../ActionCreators';

pub type NumPegsState = number;

pub default fn numPegs(
  state: NumPegsState = 0,
  action: Action,
): NumPegsState {
  switch (action.type) {
    case 'WIPE_BOARD':
      return 0;
    case 'POPULATE':
      return action.pegs.length;
    case 'MOVE':
      return state - 1;
    default:
      return state;
  }
}

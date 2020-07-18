use type { Action } from '../ActionCreators';

pub type PhaseState = 'ready' | 'picking' | 'excited';

pub default fn excited(
  state: PhaseState = 'ready',
  action: Action,
): PhaseState {
  switch (action.type) {
    case 'WIPE_BOARD':
      return 'ready';
    case 'POPULATE':
      return 'picking';
    case 'EXCITE':
      return 'excited';
    default:
      return state;
  }
}

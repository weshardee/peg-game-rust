use type { Action } from '../ActionCreators';

pub type ExcitedState = ?string;

pub default fn excited(
  state: ExcitedState = null,
  action: Action,
): ExcitedState {
  switch (action.type) {
    case 'EXCITE':
      return action.id;
    default:
      return null;
  }
}

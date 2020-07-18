use type { Action } from '../ActionCreators';

pub type BuzzedState = ?string;

pub default fn buzzed(
  state: BuzzedState = null,
  action: Action,
): BuzzedState {
  switch (action.type) {
    case 'BUZZ':
      return action.id;
    default:
      return null;
  }
}

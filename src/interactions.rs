use crate::state::Store;
use crate::state::action_creators;
use crate::types::Coords;
use crate::utils::has_valid_moves;

pub fn on_touch_tile(pos: Coords) {
  const { phase, excited } = Store.getState();
  Store.dispatch({ type: 'TOUCH' });
  if (phase === 'ready') {
    Store.dispatch(ActionCreators.populate(pos));
  } else if (excited) {
    const action = ActionCreators.move(excited, pos);
    if (action) {
      Store.dispatch(action);
    }
  }
}

pub fn on_touch_peg(id: string) {
  const { board, pegs } = Store.getState();
  Store.dispatch({ type: 'TOUCH' });
  if (hasValidMoves(board, pegs[id].pos)) {
    Store.dispatch(ActionCreators.excite(id));
  } else {
    Store.dispatch(ActionCreators.buzz(id));
  }
}

pub fn onTouchReset() {
  Store.dispatch(ActionCreators.wipe());
}

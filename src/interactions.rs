use crate::action;
use crate::store::Store;
use crate::types::Action;
use crate::types::Coords;
use crate::types::Phase;

pub fn on_touch_tile(store: Store, pos: Coords) {
  let state = store.state();
  let phase = state.phase;
  let excited = state.excited;
  store.dispatch(Action::Touch);
  if phase == Phase::Ready {
    store.dispatch(action::populate(store, pos));
  } else if excited {
    store.dispatch(action::go(store, excited, pos));
  }
}

pub fn on_touch_peg(store: Store, id: String) {
  let state = store.state();
  let board = state.board;
  let pegs = state.pegs;
  store.dispatch(Action::Touch);
  if (has_valid_moves(board, pegs[id].pos)) {
    store.dispatch(action::excite(id));
  } else {
    store.dispatch(action::buzz(id));
  }
}

pub fn on_touch_reset() {
  store.dispatch(action::wipe());
}

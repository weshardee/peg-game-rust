use crate::store::Store;
use crate::types::Action;
use crate::types::{Coords, Peg, PegType};
use crate::utils::*;

pub fn wipe() -> Action {
  Action::Wipe
}

pub fn populate(store: Store, empty_pos: Coords) -> Action {
  let pegs = Vec::new();
  for position in store.state().board.ier() {
    if !are_equal(position, empty_pos) {
      pegs.push(make_peg(position));
    }
  }
  return Action::Populate(pegs);
}

pub fn excite(id: String) -> Action {
  Action::Excite(id)
}

pub fn buzz(id: String) -> Action {
  Action::Buzz(id)
}

pub fn go(store: Store, peg_id: String, to: Coords) -> Action {
  let state = store.state();
  let pegs = &state.pegs;
  let board = &state.board;
  let from = pegs[peg_id].pos;
  if !from || board.get(to) {
    return buzz(peg_id);
  }
  let middlePos = get_middle_position(from, to);
  let middleID = board.get(middlePos);
  if !middleID {
    return buzz(peg_id);
  }
  let kill = pegs[middleID].unwrap();
  return Action::Move {
    id: peg_id,
    from,
    to,
    kill,
  };
}

// -----------------------------------------------------------------------------
// helpers
// -----------------------------------------------------------------------------

fn random_peg_type() -> PegType {
  rand::random()
}

static peg_ids: u8 = 0;

fn make_peg(pos: Coords) -> Peg {
  let id = peg_ids;
  peg_ids += 1;
  return Peg {
    id: id.to_string(),
    peg_type: random_peg_type(),
    pos,
  };
}

// TODO
// fn isGameOver() {
// self._pegsGroup.forEach(sprite => {
//   let pos = screenToBoardPosition(sprite);
//   sprite.alive = self.state.board.hasValidMoves(pos);
// });
//
// if (self._pegsGroup.getFirstAlive() == null) {
//   self.end();
// }
// }

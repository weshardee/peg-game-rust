use crate::board::Board;
use crate::constants::BOARD_SIZE;
use crate::types::Action;
use crate::types::Peg;
use crate::types::Phase;
use std::collections::HashMap;

pub struct State {
  pub board: Board,
  pub buzzed: BuzzedState,
  pub excited: ExcitedState,
  pub pegs: PegsState,
  pub num_pegs: NumPegsState,
  pub phase: Phase,
}

pub type BuzzedState = Option<String>;
pub type ExcitedState = Option<String>;
pub type NumPegsState = usize;
pub type PegsState = HashMap<String, Peg>;

// -----------------------------------------------------------------------------
// reducers
// -----------------------------------------------------------------------------

pub fn reduce(state: State, action: Action) -> State {
  State {
    board: reduce_board(state.board, action),
    pegs: reduce_pegs(state.pegs, action),
    num_pegs: reduce_num_pegs(state.num_pegs, action),
    excited: reduce_excited(state.excited, action),
    phase: reduce_phase(state.phase, action),
    buzzed: reduce_buzzed(state.buzzed, action),
  }
}

pub fn reduce_board(board: Board, action: Action) -> Board {
  match action {
    Action::Wipe => Board::new(BOARD_SIZE),
    Action::Populate(pegs) => {
      let mut nextBoard = Board::new(BOARD_SIZE);
      for peg in pegs {
        nextBoard = nextBoard.set(peg.pos, peg.id);
      }
      nextBoard
    }
    Action::Move { id, to, kill, from } => {
      return board.set(to, id).set(kill.pos, None).set(from, None);
    }
    _ => return board,
  }
}

pub fn reduce_buzzed(state: BuzzedState, action: Action) -> BuzzedState {
  match action {
    Action::Buzz(id) => id,
    _ => None,
  }
}

pub fn reduce_excited(state: ExcitedState, action: Action) -> ExcitedState {
  match (action) {
    Action::Excite(id) => id,
    _ => None,
  }
}

pub fn reduce_num_pegs(state: NumPegsState, action: Action) -> NumPegsState {
  match action {
    Action::Wipe => 0,
    Action::Populate(pegs) => pegs.length,
    Action::Move => state - 1,
    _ => state,
  }
}

pub fn reduce_pegs(prev: PegsState, action: Action) -> PegsState {
  match action {
    Action::Populate(pegs) => {
      let nextPegs = Default::default();
      for peg in pegs {
        nextPegs[peg.id] = peg;
      }
      nextPegs
    }
    Action::Wipe => Vec::new(), // TODO size?
    Action::Move { id, kill, from, to } => {
      let mut next: PegsState = HashMap { ..prev };
      // TODO
      // next.remove(action.kill.id);
      // next.set(id, Peg {
      //   pos: to,
      //   ..prev[id],
      // });
      next
    }
    _ => pegs,
  }
}

pub fn reduce_phase(state: Phase, action: Action) -> Phase {
  match action {
    Action::Wipe => Phase::Ready,
    Action::Populate => Phase::Picking,
    Action::Excite => Phase::Excited,
    _ => state,
  }
}


pub use crate::reducers::board::BoardState;
pub use crate::reducers::buzzed::BuzzedState;
pub use crate::reducers::excited::ExcitedState;
pub use crate::reducers::pegs::PegsState;
pub use crate::reducers::numPegs::NumPegsState;
pub use crate::reducers::phase::PhaseState;

// -----------------------------------------------------------------------------
// reducers
// -----------------------------------------------------------------------------

use { combineReducers } from 'redux';

use board from 'crate::reducers::board';
use pegs from 'crate::reducers::pegs';
use numPegs from 'crate::reducers::numPegs';
use excited from 'crate::reducers::excited';
use phase from 'crate::reducers::phase';
use buzzed from 'crate::reducers::buzzed';

pub fn reduce(state: State, action: Action) -> State {
  let state = board(state, action);
  let state = pegs(state, action);
  let state = numPegs(state, action);
  let state = excited(state, action);
  let state = phase(state, action);
  let state = buzzed(state, action);
  state
});

// -----------------------------------------------------------------------------
// selectors
// -----------------------------------------------------------------------------


// aggregate types for easy access from other modules
pub pub type {
  BoardState,
    BuzzedState,
    ExcitedState,
    PegsState,
    NumPegsState,
    PhaseState,
};

pub struct State = {
  board: BoardState,
  buzzed: BuzzedState,
  excited: ExcitedState,
  pegs: PegsState,
  numPegs: NumPegsState,
  phase: PhaseState,
};

pub const getBoard = (state: State): BoardState => state.board;
pub const getPegs = (state: State): PegsState => state.pegs;
pub const getBuzzed = (state: State): BuzzedState => state.buzzed;
pub const getExcited = (state: State): ExcitedState => state.excited;
pub const getNumPegs = (state: State): NumPegsState => state.numPegs;
pub const getPhase = (state: State): PhaseState => state.phase;

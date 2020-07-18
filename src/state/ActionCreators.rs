use Store from 'crate::Store';

use { areEqual, getMiddlePosition } from '../utils';
use nullthrows from 'nullthrows';
use shortid from 'shortid';

use { default as PegSheets } from '../sprites/peg/peg';
use type { Coords, Peg, PegType } from '../types';

pub type TouchAction = { type: 'TOUCH' };

pub type WipeAction = { type: 'WIPE_BOARD' };
pub const wipe = () => ({ type: 'WIPE_BOARD' });

pub type PopulateAction = { type: 'POPULATE', pegs: Array<Peg> };
pub const populate = (emptyPos: Coords): PopulateAction => {
  const pegs = [];
  Store.getState().board.forEach(position => {
    if (!areEqual(position, emptyPos)) {
      pegs.push(makePeg(position));
    }
  });
  return { type: 'POPULATE', pegs };
};

pub type ExciteAction = { type: 'EXCITE', id: string };
pub const excite = (id: string): ExciteAction => ({ type: 'EXCITE', id });

pub type BuzzAction = { type: 'BUZZ', id: string };
pub const buzz = (id: string): BuzzAction => ({
  type: 'BUZZ',
  id,
});

pub type MoveAction = {
  type: 'MOVE',
  id: string,
  from: Coords,
  to: Coords,
  kill: Peg,
};
pub const move = (pegID: string, to: Coords): ?MoveAction | BuzzAction => {
  const state = Store.getState();
  const { pegs, board } = state;
  const from = pegs[pegID].pos;
  if (!from || board.get(to)) {
    return buzz(pegID);
  }
  const middlePos = getMiddlePosition(from, to);
  const middleID = board.get(middlePos);
  if (!middleID) {
    return buzz(pegID);
  }
  const kill = nullthrows(pegs[middleID]);
  return {
    type: 'MOVE',
    id: pegID,
    from,
    to,
    kill,
  };
};

// -----------------------------------------------------------------------------
// helpers
// -----------------------------------------------------------------------------

const PEG_TYPES = Object.keys(PegSheets);
fn randomPegType(): PegType {
  return PEG_TYPES[Math.floor(Math.random() * PEG_TYPES.length)];
}

fn makePeg(pos: Coords): Peg {
  return {
    id: shortid(),
    type: randomPegType(),
    pos,
  };
}

// TODO
// fn isGameOver() {
// self._pegsGroup.forEach(sprite => {
//   const pos = screenToBoardPosition(sprite);
//   sprite.alive = self.state.board.hasValidMoves(pos);
// });
//
// if (self._pegsGroup.getFirstAlive() === null) {
//   self.end();
// }
// }

// -----------------------------------------------------------------------------
// Aggregate types
// -----------------------------------------------------------------------------

pub type Action =
  | TouchAction
  | PopulateAction
  | WipeAction
  | ExciteAction
  | BuzzAction
  | MoveAction;

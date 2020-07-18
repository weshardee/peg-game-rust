use { BOARD_SIZE } from '../../constants';
use Board from '../../Board';

use type { Action } from '../ActionCreators';

pub type BoardState = Board<string>;

pub default fn board(
  board: BoardState = new Board(BOARD_SIZE),
  action: Action,
): Board<string> {
  switch (action.type) {
    case 'WIPE_BOARD':
      return new Board(BOARD_SIZE);
    case 'POPULATE':
      let nextBoard = new Board(BOARD_SIZE);
      for (let { pos, id } of action.pegs) {
        nextBoard = nextBoard.set(pos, id);
      }
      return nextBoard;
    case 'MOVE':
      return board
        .set(action.to, action.id)
        .set(action.kill.pos, null)
        .set(action.from, null);
    default:
      return board;
  }
}

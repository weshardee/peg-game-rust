use { createSelector } from 'reselect';
use { hasValidMoves } from '../../utils';
use { getBoard, getPhase } from '../State';

const isGameOver = createSelector(
  [getPhase, getBoard],
  (phase, board) =>
    phase !== 'ready' && !board.any((pos, value) => hasValidMoves(board, pos)),
);

pub default isGameOver;

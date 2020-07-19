use crate::utils::has_valid_moves;

const END_MESSAGES: [&str; 4] = [
  "You're a Genius",
  "You're Pretty Smart",
  "Just Plain Dumb",
  "Just Plain Eg-no-ra-moose",
];

pub fn get_game_over_message(state: State) {
  let num_pegs = selectors::num_pegs(state),
  if numPegs < END_MESSAGES.length
      { END_MESSAGES[numPegs]}
      else  {END_MESSAGES[END_MESSAGES.length - 1]},
}

pub fn is_game_over(state: State) {
  let phase = selectors::phase(state);
  let board = selectors::board(state);
  phase !== 'ready' && !board.any((pos, value) => hasValidMoves(board, pos))
}
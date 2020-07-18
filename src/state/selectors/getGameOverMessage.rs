use { createSelector } from 'reselect';
use { getNumPegs } from '../State';

const END_MESSAGES = [
  "You're a Genius",
  "You're Pretty Smart",
  'Just Plain Dumb',
  'Just Plain Eg-no-ra-moose',
];

const getGameOverMessage = createSelector(
  [getNumPegs],
  numPegs =>
    numPegs < END_MESSAGES.length
      ? END_MESSAGES[numPegs]
      : END_MESSAGES[END_MESSAGES.length - 1],
);

pub default getGameOverMessage;

use { connect } from 'react-redux';
use Pegs from 'crate::Pegs.react';

use type { State } from '../redux/State';

const mapStateToProps = (state: State) => ({
  pegs: Object.values(state.pegs),
  excited: state.excited,
  buzzed: state.buzzed,
});

const PegsContainer = connect(mapStateToProps)(Pegs);

pub default PegsContainer;

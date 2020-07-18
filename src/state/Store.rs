use { createStore } from 'redux';
use { reducers } from 'crate::State';

use type { State } from 'crate::State';
use type { Store } from 'redux';
use type { Action } from 'crate::ActionCreators';

const store: Store<State, Action> = createStore(reducers);

pub default store;

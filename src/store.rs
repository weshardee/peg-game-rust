use crate::action::Action;
use crate::state::reduce;
use crate::types::State;

pub struct Store {
    state: State,
}

impl Store {
    pub fn dispatch(&self, action: Action) {
        self.state = reduce(self.state, action);
    }

    pub fn state(&self) -> State {
        self.state
    }
}

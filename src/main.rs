mod board;
mod constants;
mod interactions;
mod types;
mod utils;

const TITLE: &str = "Peg Game";

struct App {
    state: GameState,
}

impl KApp for App {
    fn new() -> Self {
        Self {
            state: Default::default(),
        }
    }
    fn init(&mut self, ctx: &mut Ctx) {
        let state = &mut self.state;
        game::load(ctx, state);
    }
    fn frame(&mut self, ctx: &mut Ctx) {
        let state = &mut self.state;
        game::update(ctx, state, TARGET_DT);
        game::draw(ctx, state);
    }
}

fn main() {
    run::<App>(KAppDesc {
        window_title: TITLE.to_string(),
        ..Default::default()
    });
}

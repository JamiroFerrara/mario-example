struct Player {
    state: State,
}

impl Player {
    fn new() -> Self {
        Self {
            state: State::Mario,
        }
    }
    fn collect(&mut self, power: Transition) {
        match (&self.state, power) {
            (State::Mario, Transition::Feather) => self.state = State::SuperMario,
            (_, Transition::Flower) => self.state = State::FireMario,
            (_, Transition::Feather) => self.state = State::CapeMario,
            (_, Transition::Mushroom) => {}
        }
    }
}

enum State {
    Mario,
    SuperMario,
    FireMario,
    CapeMario,
}

enum Transition {
    Feather,
    Flower,
    Mushroom,
}

fn main() {
    let mut mario = Player::new();
    mario.collect(Transition::Mushroom);
    mario.collect(Transition::Flower);
    mario.collect(Transition::Feather);
}

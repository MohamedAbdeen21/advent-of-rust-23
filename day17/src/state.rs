use std::cmp::Ordering;

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct State {
    pub x: usize,
    pub y: usize,
    pub direction: (isize, isize),
    pub steps: usize, // steps in the same direction
    pub heatloss: u64,
    // trace the route
    // Yes, it's not a reference, and I'll have to clone every
    // time. I don't want to deal with lifetimes and references
    // now, maybe later.
    pub parent: Option<Box<State>>,
}

impl State {
    pub fn add_loss(&self, loss: u8) -> State {
        let mut clone = self.clone();
        clone.heatloss += loss as u64;
        return clone;
    }

    pub fn hash(&self) -> State {
        let mut clone = self.clone();
        clone.heatloss = 0;
        clone.parent = None;
        return clone;
    }

    pub fn new(x: usize, y: usize, dx: isize, dy: isize, steps: usize, heatloss: u64) -> State {
        State {
            x,
            y,
            direction: (dx, dy),
            steps,
            heatloss,
            parent: None,
        }
    }

    pub fn step(&self) -> State {
        State {
            x: self.x.wrapping_add_signed(self.direction.0),
            y: self.y.wrapping_add_signed(self.direction.1),
            direction: self.direction,
            steps: self.steps + 1,
            heatloss: self.heatloss,
            parent: Some(Box::new(self.clone())),
        }
    }

    pub fn turn(&self, direction: (isize, isize)) -> State {
        let new = State {
            x: self.x.wrapping_add_signed(direction.0),
            y: self.y.wrapping_add_signed(direction.1),
            direction,
            steps: 1,
            heatloss: self.heatloss,
            parent: Some(Box::new(self.clone())),
        };
        return new;
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.heatloss.cmp(&self.heatloss)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

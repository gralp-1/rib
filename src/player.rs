pub struct Player {
    pub pos: (i32, i32),
}

impl Player {
    pub fn new(pos: (i32, i32)) -> Self {
        Player {
            pos
        }
    }
}
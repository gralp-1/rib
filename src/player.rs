pub struct Player {
    health: i32,
    x: i32,
    y: i32
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            health: 100,
            x,
            y
        }
    }
    pub fn get_health(&self) -> i32 {
        self.health
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn try_up(&mut self) {
        // absolute value of x-1
        self.x = (self.x - 1).abs();
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }
}
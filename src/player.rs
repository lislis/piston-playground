pub struct Player {
    pub rows: i32,
    pub columns: i32,
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn new() -> Player {
        Player {
            rows: 4,
            columns: 3,
            x: 0,
            y: 0
        }
    }
    pub fn moving(&mut self, x: i32, y: i32) {
        self.x += x;
        if self.x > self.columns -1 {
            self.x = 0;
        } else if self.x < 0 {
            self.x = self.columns -1;
        }
        self.y += y;
        if self.y > self.rows -1 {
            self.y = 0;
        } else if self.y < 0 {
            self.y = self.rows -1;
        }
    }

}

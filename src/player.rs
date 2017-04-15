use apples::Apples;
use throwable::Throwable;

pub struct Player {
    pub rows: i32,
    pub columns: i32,
    pub x: i32,
    pub y: i32,
    pub apples: Apples,
    pub throwable: Vec<Throwable>
}

impl Player {
    pub fn new() -> Player {
        Player {
            rows: 4,
            columns: 3,
            x: 0,
            y: 0,
            apples: Apples::new(),
            throwable: vec![]
        }
    }

    pub fn throw(&mut self, x_pos: f64, y_pos: f64) {
        if self.apples.left > 0 {
            self.throwable.push(Throwable::new(x_pos, y_pos));
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

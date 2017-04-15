pub struct Throwable {
    pub active: bool,
    pub x: f64,
    pub y: f64,
}

impl Throwable {
    pub fn new(x_pos: f64, y_pos: f64) -> Throwable {
        Throwable {
            active: true,
            x: x_pos,
            y: y_pos
        }
    }
    pub fn set_active(&mut self, x: f64, y: f64) {
        self.active = true;
        self.x = x;
        self.y = y;
    }
    pub fn update(&mut self, delta_time: f64) {
        if self.active {
            self.y += delta_time;
        }
    }
}

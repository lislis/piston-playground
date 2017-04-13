pub struct Apples {
    pub total: i32,
    pub gone: i32,
    pub left: i32
}

impl Apples {
    pub fn new() -> Apples {
        Apples {total: 10, gone: 0, left: 10}
    }
    pub fn is_gone(&mut self, i: i32) {
        self.gone += i;
        self.left -= i;
    }
}

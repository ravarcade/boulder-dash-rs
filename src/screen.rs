pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub scr: Vec<u8>,
}

impl Screen {
    pub fn new(w: usize, h: usize, ch: u8) -> Self {
        Self::clear();
        let scr = vec![ch; w * h];
        Self { w, h, scr }
    }

    pub fn reset(&mut self, w: usize, h: usize, ch: u8) {
        Self::clear();
        self.scr = vec![ch; w * h];
        self.w = w;
        self.h = h;
    }

    pub fn draw(&self) {
        Self::set_cursor_position(0, 0);
        for y in 0..self.h {
            let l: usize = y * self.w;
            let r = l + self.w;
            let s = std::str::from_utf8(&self.scr[l..r]).unwrap();
            println!("{}", s);
        }
    }

    pub fn drawDbg(&self) {
        Self::clear();
        self.draw();
    }

    pub fn put(&mut self, x: usize, y: usize, ch: u8) {
        self.scr[y * self.w + x] = ch;
    }

    pub fn get(&self, i: usize, d: isize, def: u8) -> u8 {
        let i = i.wrapping_add_signed(d);
        *self.scr.get(i).unwrap_or(&def)
    }

    pub fn set_cursor_position(x: u32, y: u32) {
        print!("\x1b[{};{}H", y, x);
    }

    pub fn clear() {
        print!("!\x1b[2J");
    }

    pub fn hline(&mut self, y: usize, x1: usize, x2: usize, ch: u8) {
        for x in x1..x2 {
            self.put(x, y, ch);
        }
    }

    pub fn vline(&mut self, x: usize, y1: usize, y2: usize, ch: u8) {
        for y in y1..y2 {
            self.put(x, y, ch);
        }
    }

}

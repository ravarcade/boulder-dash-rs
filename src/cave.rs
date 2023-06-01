use crossterm::event::{
    poll,
    read,
    Event,
    KeyCode,
    KeyEvent,
    KeyEventKind,
};

use crate::screen::Screen;

pub const DIRT: u8 = b'.';
pub const BORDER: u8 = b'@';
pub const BOULDER: u8 = b'O';
pub const DIAMOND: u8 = b'$';
pub const WALL: u8 = b'W';
pub const EMPTY: u8 = b' ';
pub const SPAWN: u8 = b'S';
pub const PLAYER: u8 = b'R';
pub const BUTTERFLY: u8 = b'E';
pub const EXIT: u8 = b'_';

const BD_SPACE: u8 = 0x00;
const BD_DIRT: u8 = 0x01;
const BD_BRICK: u8 = 0x02;
const BD_MAGIC: u8 = 0x03;
const BD_STEEL: u8 = 0x07;
const BD_FIREFLY: u8 = 0x08;
const BD_BOULDER: u8 = 0x10;
const BD_DIAMOND: u8 = 0x14;
const BD_BUTTERFLY: u8 = 0x30;
const BD_ROCKFORD: u8 = 0x38;
const BD_AMOEBA: u8 = 0x3A;
const BD_PREROCKFORD: u8 = 0x25;
const BD_EXIT: u8 = 0x04;

fn conv(o: u8) -> u8 {
    match o {
        BD_SPACE => EMPTY,
        BD_DIRT => DIRT,
        BD_BRICK => WALL,
        BD_MAGIC => b'?',
        BD_STEEL => BORDER,
        BD_FIREFLY => b'?',
        BD_BOULDER => BOULDER,
        BD_DIAMOND => DIAMOND,
        BD_BUTTERFLY => BUTTERFLY,
        BD_PREROCKFORD => SPAWN,
        BD_ROCKFORD => PLAYER,
        BD_EXIT => b'_',
        BD_AMOEBA => b'?',
        _ => b'_',
    }
}
// see: https://github.com/jakesgordon/javascript-boulderdash/blob/master/caves.js

fn wait_key()
{
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '), 
                kind: KeyEventKind::Press,
                ..
            }) => break,
            _ => (),
        }
    }
}

struct Rect {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}
struct CaveBuilder {
    random_seed: u8,
    random_objects: [u8; 4],
    random_object_prob: [u8; 4],
    width: usize,
    height: usize,
}

impl CaveBuilder {
    pub fn new(cave: &Vec<u8>) -> Self {
        let mut ro = [cave[0x18], cave[0x19], cave[0x1A], cave[0x1B]];
        for o in &mut ro {
            *o = conv(*o);
        }
        Self {
            width: 40,
            height: 22,
            random_seed: cave[0x04],
            random_objects: ro,
            random_object_prob: [cave[0x1C], cave[0x1D], cave[0x1E], cave[0x1F]],
        }
    }
}

impl Screen {
    pub fn load_cave(&mut self, cave: &Vec<u8>) {
        let cb = CaveBuilder::new(&cave);
        let playfield = Rect {
            x: 0,
            y: 0,
            w: cb.width,
            h: cb.height,
        };

        self.reset(cb.width, cb.height, EMPTY);
        Self::set_cursor_position(0, (cb.height + 1) as u32);
        let mut seeds = [0, cb.random_seed];
        for y in 1..cb.height - 1 {
            for x in 0..cb.width {
                let mut ch = DIRT;
                Self::bdrandom(&mut seeds);
                for n in 0..cb.random_objects.len() {
                    if seeds[0] < cb.random_object_prob[n] {
                        ch = cb.random_objects[n];
                    }
                }
                self.put(x, y, ch);
            }
        }

        self.rect_border(BORDER, &playfield);

        let mut n = 0x20;
        while (n < cave.len()) && (cave[n] < 0xFF) {
            self.drawDbg();
            let ch = conv(cave[n] & 0x3F); //  low 6 bits
            let kind = cave[n] >> 6; // high 2 bits
            match kind {
                0 => {
                    let x = cave[n + 1] as usize;
                    let y = cave[n + 2] as usize - 2; // raw data assumes top 2 lines are for displaying scores
                    println!("put: {x}, {y}, {ch} ({})", cave[n] & 0x3f);
                    self.put(x, y, ch);
                    n = n + 3;
                }
                1 => {
                    println!("draw_line: ch: {ch}, {:?}", &cave[n + 1..n + 5]);
                    self.draw_line(ch, &cave[n + 1..n + 5]);
                    n = n + 5;
                }
                2 => {
                    let ch2 = conv(cave[n + 5] & 0x3f);
                    self.draw_filed_rect(ch, ch2, Self::to_rect(&cave[n + 1..n + 5]));
                    println!(
                        "draw_filed_rect: ch: {}/{}, ch2: {}/{}, {:?}",
                        ch,
                        cave[n],
                        ch2,
                        cave[n + 5],
                        &cave[n + 1..n + 5]
                    );
                    n = n + 6;
                }
                3 => {
                    self.draw_rect(ch, Self::to_rect(&cave[n + 1..n + 5]));
                    println!("draw_rect: ch: {ch}, {:?}", &cave[n + 1..n + 5]);
                    n = n + 5;
                }
                _ => panic!("Will not happend"),
            };
            wait_key();
        }

        self.draw();
    }

    fn to_rect(cave: &[u8]) -> Rect {
        Rect {
            x: cave[0] as usize,
            y: cave[1] as usize - 2,
            w: cave[2] as usize,
            h: cave[3] as usize,
        }
    }

    fn draw_line(&mut self, ch: u8, cave: &[u8]) {
        const DIRX: &[i32; 8] = &[0, 1, 1, 1, 0, -1, -1, -1];
        const DIRY: &[i32; 8] = &[-1, -1, 0, 1, 1, 1, 0, -1];
        let dx = DIRX[cave[3] as usize] as isize;
        let dy = DIRY[cave[3] as usize] as isize;
        let mut x = cave[0] as isize;
        let mut y = cave[1] as isize - 2;
        for _i in 0..cave[2] {
            self.put(x as usize, y as usize, ch);
            println!("p[{}, {}] + [{}, {}]", x, y, dx, dy);
            x = x + dx;
            y = y + dy;
        }
    }

    fn draw_rect(&mut self, ch: u8, rect: Rect) {
        self.rect_filed(ch, &rect);
    }

    fn draw_filed_rect(&mut self, ch: u8, ch2: u8, rect: Rect) {
        self.rect_border(ch, &rect);
        self.draw_rect(
            ch2,
            Rect {
                x: rect.x + 1,
                y: rect.y + 1,
                w: rect.w - 2,
                h: rect.h - 2,
            },
        );
    }

    fn rect_filed(&mut self, ch: u8, rect: &Rect) {
        for y in rect.y..(rect.y + rect.h) {
            println!(
                "hline: y = {}, x = {} .. {}, ch = {}",
                y,
                rect.x,
                rect.x + rect.w,
                ch
            );
            self.hline(y, rect.x, rect.x + rect.w, ch);
        }
    }

    fn rect_border(&mut self, ch: u8, rect: &Rect) {
        self.hline(rect.y, rect.x, rect.x + rect.w, ch);
        self.hline(rect.y + rect.h - 1, rect.x, rect.x + rect.w, ch);
        self.vline(rect.x, rect.y + 1, rect.y + rect.h - 1, ch);
        self.vline(rect.x + rect.w - 1, rect.y + 1, rect.y + rect.h - 1, ch);
    }

    fn ror1b(v: u8) -> u8 {
        (v & 0x01) << 7
    }

    fn bdrandom(seeds: &mut [u8; 2]) {
        let [s0, s1] = seeds.to_owned();
        let tmp1 = Self::ror1b(s0);
        let tmp2 = s1 >> 1;
        let mut result: u16 = s1 as u16 + Self::ror1b(s1) as u16;
        result = (result & 0xff) + (result >> 8) + 0x13;
        seeds[1] = (result & 0xff) as u8;

        result = s0 as u16 + (result >> 8) + tmp1 as u16;
        result = (result & 0xff) + (result >> 8) + tmp2 as u16;
        seeds[0] = (result & 0xff) as u8;
    }
}

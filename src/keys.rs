use crossterm::event::{
    poll,
    read,
    Event,
    KeyCode,
    KeyEvent,
};
use std::time;

#[derive(Debug)]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub struct Keys {
    pub dir: Direction,
    pub fire: bool,
    pub esc: bool,
    pub key: KeyCode,
}

impl Keys {
    pub fn new() -> Self {
        Self {
            dir: Direction::None,
            fire: false,
            esc: false,
            key: KeyCode::Null,
        }
    }

    fn is_event_available() -> bool {
        poll(time::Duration::from_secs(0)).unwrap_or(false) == true
    }

    fn clear(&mut self) {
        self.dir = Direction::None;
        self.fire = false;
        self.esc = false;
        self.key = KeyCode::Null;
    }

    pub fn read(&mut self) {
        self.clear();
        while Self::is_event_available() {
            let event = read().unwrap();
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => self.dir = Direction::Left,
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => self.dir = Direction::Right,
                Event::Key(KeyEvent {
                    code: KeyCode::Up, ..
                }) => self.dir = Direction::Up,
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    ..
                }) => self.dir = Direction::Down,
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => self.fire = true,
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => self.esc = true,
                Event::Key(KeyEvent {
                    code, ..
                }) => self.key = code,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_any_key() {
        let mut keys = Keys::new();
        println!("press left arrow");
        loop {
            keys.read();
            match keys.dir {
                Direction::Left => break,
                _ => {}
            }
        }

        assert_eq!(true, true)
    }
}

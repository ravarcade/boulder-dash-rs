use std::{thread, time};
mod keys;
mod screen;
mod boulder_dash;
mod cave;

fn main() {
    let frame_time = time::Duration::from_millis(150);
    let mut game = boulder_dash::Game::new();
    game.start_level();
    while game.process()
    {
        thread::sleep(frame_time);
    }
    
}


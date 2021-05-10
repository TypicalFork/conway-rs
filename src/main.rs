use conway::game::State;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};
use termion::{
    clear,
    screen::{ToAlternateScreen, ToMainScreen},
};

fn main() {
    let terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate)).unwrap();

    print!("{}", ToAlternateScreen);
    let mut state = State::random(0.2, (67, 40));

    while !terminate.load(Ordering::Relaxed) {
        print!("{}", clear::All);
        print!("{}", state);

        thread::sleep(time::Duration::from_millis(200));
        state = state.next();
    }

    print!("{}", ToMainScreen);
}

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
    terminal_size,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate)).unwrap();

    print!("{}", ToAlternateScreen);
    let size = terminal_size()?;
    // Each pixel is two characters and so the effective width of the terminal is halved.
    let resolution = ((size.0 / 2) as usize, size.1 as usize);
    // The calculated state is larger than the resolution so that the pixels still properly work
    // near the borders and off screen. Obviously this isn't a true representation of the infinite
    // grid of conway's game of life, but it's good enough.
    let mut state = State::random(0.2, (resolution.0 + 60, resolution.1 + 60));

    while !terminate.load(Ordering::Relaxed) {
        print!("{}", clear::All);
        print!("{}", state.center(resolution));

        thread::sleep(time::Duration::from_millis(100));
        state = state.next();
    }

    print!("{}", ToMainScreen);
    Ok(())
}

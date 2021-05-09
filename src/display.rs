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

pub fn display(terminate: Arc<AtomicBool>) {
    print!("{}", ToAlternateScreen);

    let mut i = 0;
    while !terminate.load(Ordering::Relaxed) {
        i += 1;
        i %= 2;

        print!("{}", clear::All);

        for r in 1..20 {
            for c in 1..44 {
                if (r % 2 == 1) ^ (c % 2 == i) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        thread::sleep(time::Duration::from_millis(200));
    }

    print!("{}", ToMainScreen);
}

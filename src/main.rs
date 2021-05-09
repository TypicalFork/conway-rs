use conway::display::display;
use std::sync::{atomic::AtomicBool, Arc};

fn main() {
    let terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&terminate)).unwrap();
    display(Arc::clone(&terminate));
}

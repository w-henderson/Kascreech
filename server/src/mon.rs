use humphrey::monitor::event::Event;

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc::Receiver;

pub fn monitor(rx: Receiver<Event>) {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("kascreech.log")
        .unwrap();

    for event in rx {
        writeln!(log_file, "{}", event).unwrap();
    }
}

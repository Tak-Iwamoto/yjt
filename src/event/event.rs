use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};

use super::key::Key;

#[derive(Debug, Clone, Copy)]
pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: mpsc::Sender<Event<Key>>,
}

const TICK_RATE: Duration = Duration::from_millis(250);

impl Events {
    /// Constructs an new instance of `Events` with the default config.
    pub fn new() -> Events {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();

        thread::spawn(move || loop {
            if event::poll(TICK_RATE).unwrap() {
                if let event::Event::Key(key) = event::read().unwrap() {
                    let key = Key::from(key);

                    event_tx.send(Event::Input(key)).unwrap();
                }
            }

            event_tx.send(Event::Tick).unwrap();
        });

        Events { rx, _tx: tx }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }
}

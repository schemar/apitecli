use super::Key;
use crossterm::event;
use std::{sync::mpsc, thread, time::Duration};

#[derive(Debug, Clone, Copy)]
/// Configuration for event handling.
pub struct EventConfig {
    /// The tick rate at which the application will sent an tick event.
    pub tick_rate: Duration,
}

impl Default for EventConfig {
    fn default() -> EventConfig {
        EventConfig {
            tick_rate: Duration::from_millis(250),
        }
    }
}

/// A small event handler that wrap crossterm input and tick event. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct KeyEvents {
    rx: mpsc::Receiver<Key>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: mpsc::Sender<Key>,
}

impl KeyEvents {
    /// Constructs an new instance of `Events` with the default config.
    pub fn new(tick_rate: u64) -> KeyEvents {
        KeyEvents::with_config(EventConfig {
            tick_rate: Duration::from_millis(tick_rate),
        })
    }

    /// Constructs an new instance of `Events` from given config.
    pub fn with_config(config: EventConfig) -> KeyEvents {
        let (tx, rx) = mpsc::channel();

        let event_tx = tx.clone();
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if event::poll(config.tick_rate).unwrap() {
                    if let event::Event::Key(key) = event::read().unwrap() {
                        let key = Key::from(key);

                        event_tx.send(key).unwrap();
                    }
                }

                event_tx.send(Key::None).unwrap();
            }
        });

        KeyEvents { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function will block the current thread.
    pub fn next(&self) -> Result<Key, mpsc::RecvError> {
        self.rx.recv()
    }
}

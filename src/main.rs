use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

fn main() {
    let _ = enable_raw_mode();
    loop {
        if let Ok(k) = key_pressed() {
            println!("k : {k}")
        }
    }
}

fn key_pressed() -> Result<char, bool> {
    if event::poll(Duration::from_millis(100)).unwrap_or(false) {
        if let Ok(Event::Key(key_pressed)) = event::read()
            && key_pressed.kind == KeyEventKind::Press
        {
            match key_pressed.code {
                KeyCode::Char('z') => Ok('z'),
                KeyCode::Char('s') => Ok('s'),
                KeyCode::Char('d') => Ok('d'),
                KeyCode::Char('q') => Ok('q'),
                _ => Err(false),
            }
        } else {
            Err(false)
        }
    } else {
        Err(false)
    }
}

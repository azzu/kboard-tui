use std::fmt;
use std::fmt::Formatter;
use crossterm::event;
use serde::Deserialize;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Tab,
    Enter,
    Char(char),
    Ctrl(char),
    Esc,
    Unknown
}

impl From<event::KeyEvent> for Key {
    fn from(key_event: event::KeyEvent) -> Self {
        match key_event {
            event::KeyEvent {
                code: event::KeyCode::Left,
                ..
            } => Key::Left,
            event::KeyEvent {
                code: event::KeyCode::Right,
                ..
            } => Key::Right,
            event::KeyEvent {
                code: event::KeyCode::Up,
                ..
            } => Key::Up,
            event::KeyEvent {
                code: event::KeyCode::Down,
                ..
            } => Key::Down,
            event::KeyEvent {
                code: event::KeyCode::Tab,
                ..
            } => Key::Tab,
            event::KeyEvent {
                code: event::KeyCode::Enter,
                ..
            } => Key::Enter,
            event::KeyEvent {
                code: event::KeyCode::Char(c),
                ..
            } => Key::Char(c),
            _ => Key::Unknown
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Key::Up => write!(f, "\u{2191}"),
            Key::Down => write!(f, "\u{2193}"),
            Key::Left => write!(f, "\u{2190}"),
            Key::Right => write!(f, "\u{2192}"),
            Key::Enter | Key::Tab => write!(f, "<{:?}>", self),
            _ => write!(f, "{:?}", self),
        }
    }
}
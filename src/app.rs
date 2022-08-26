use tui::backend::Backend;
use tui::Frame;
use crate::config::Config;
use crate::event::Key;

pub enum Focus {
    BoardList,
    Table,
}

pub struct App {
    focus: Focus,
    pub config: Config,
    left_main_chunk_percentage: u16,
}

impl App {
    pub fn new(config: Config) -> App {
        Self {
            config: config.clone(),
            focus: Focus::BoardList,
            left_main_chunk_percentage: 15,
        }
    }

    pub fn draw<B: Backend> (&mut self, f: &mut Frame<'_, B>) -> anyhow::Result<()> {
        Ok(())
    }

    fn move_focus(&mut self, key: Key) -> anyhow::Result<EventState>
}
use crate::config::Config;

pub enum Focus {
    BoardList,
    Table,
}

pub struct App {
    focus: Focus,
    pub config: Config,
    left_main_chunk_percentage: u16,
    boards: BoardsComponent,
}
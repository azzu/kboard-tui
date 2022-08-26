use crate::config::KeyConfig;

pub struct CommandText {
    pub name: String,
    pub group: &'static str,
    pub hide_help: bool
}

impl CommandText {
    pub const fn new(name: String, group: &'static str) -> Self {
        Self {
            name,
            group,
            hide_help: false,
        }
    }
}

pub struct CommandInfo {
    pub text: CommandText,
}

impl CommandInfo {
    pub const fn new(text: CommandText) -> Self {
        Self{
            text
        }
    }
}
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use crate::event::Key;
use crate::log::LogLevel;
use structopt::StructOpt;
use serde::Deserialize;

#[derive(StructOpt, Debug)]
pub struct CliConfig {
    #[structopt(long, short, global = true)]
    config_path: Option<std::path::PathBuf>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub key_config: KeyConfig,
    #[serde(default)]
    pub log_level: LogLevel,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key_config: KeyConfig::default(),
            log_level: LogLevel::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(Serialize))]
pub struct KeyConfig {
    pub scroll_up: Key,
    pub scroll_down: Key,
    pub scroll_right: Key,
    pub scroll_left: Key,
    pub move_up: Key,
    pub move_down: Key,
    pub copy: Key,
    pub enter: Key,
    pub exit: Key,
    pub quit: Key,
    pub exit_popup: Key,
    pub focus_right: Key,
    pub focus_left: Key,
    pub focus_above: Key,
    pub focus_connections: Key,
    pub open_help: Key,
    pub filter: Key,
    pub scroll_down_multiple_lines: Key,
    pub scroll_up_multiple_lines: Key,
    pub scroll_to_top: Key,
    pub scroll_to_bottom: Key,
    pub extend_selection_by_one_cell_left: Key,
    pub extend_selection_by_one_cell_right: Key,
    pub extend_selection_by_one_cell_up: Key,
    pub extend_selection_by_one_cell_down: Key,
    pub tab_records: Key,
    pub tab_columns: Key,
    pub tab_constraints: Key,
    pub tab_foreign_keys: Key,
    pub tab_indexes: Key,
    pub tab_sql_editor: Key,
    pub tab_properties: Key,
    pub extend_or_shorten_widget_width_to_right: Key,
    pub extend_or_shorten_widget_width_to_left: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            scroll_up: Key::Char('k'),
            scroll_down: Key::Char('j'),
            scroll_right: Key::Char('l'),
            scroll_left: Key::Char('h'),
            move_up: Key::Up,
            move_down: Key::Down,
            copy: Key::Char('y'),
            enter: Key::Enter,
            exit: Key::Ctrl('c'),
            quit: Key::Char('q'),
            exit_popup: Key::Esc,
            focus_right: Key::Right,
            focus_left: Key::Left,
            focus_above: Key::Up,
            focus_connections: Key::Char('c'),
            open_help: Key::Char('?'),
            filter: Key::Char('/'),
            scroll_down_multiple_lines: Key::Ctrl('d'),
            scroll_up_multiple_lines: Key::Ctrl('u'),
            scroll_to_top: Key::Char('g'),
            scroll_to_bottom: Key::Char('G'),
            extend_selection_by_one_cell_left: Key::Char('H'),
            extend_selection_by_one_cell_right: Key::Char('L'),
            extend_selection_by_one_cell_down: Key::Char('J'),
            extend_selection_by_one_cell_up: Key::Char('K'),
            tab_records: Key::Char('1'),
            tab_properties: Key::Char('2'),
            tab_sql_editor: Key::Char('3'),
            tab_columns: Key::Char('4'),
            tab_constraints: Key::Char('5'),
            tab_foreign_keys: Key::Char('6'),
            tab_indexes: Key::Char('7'),
            extend_or_shorten_widget_width_to_right: Key::Char('>'),
            extend_or_shorten_widget_width_to_left: Key::Char('<'),
        }
    }
}

impl Config {
    pub fn new(config: &CliConfig) -> anyhow::Result<Self> {
        let config_path = if let Some(config_path) = &config.config_path {
            config_path.clone()
        } else {
            get_app_config_path()?.join("config.toml")
        };
        if let Ok(file) = File::open(config_path) {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;

            let config: Result<Config, toml::de::Error> = toml::from_str(&contents);
            match config {
                Ok(config) => return Ok(config),
                Err(e) => panic!("fail to parse config file: {}", e),
            }
        }
        Ok(Config::default())
    }
}

pub fn get_app_config_path() -> anyhow::Result<std::path::PathBuf> {
    let mut path = if cfg!(target_os = "macos") {
        dirs_next::home_dir().map(|h| h.join(".config"))
    } else {
        dirs_next::config_dir()
    }
        .ok_or_else(|| anyhow::anyhow!("failed to find os config dir."))?;

    path.push("kboard-tui");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn expand_path(path: &Path) -> Option<PathBuf> {
    let mut expanded_path = PathBuf::new();
    let mut path_iter = path.iter();
    if path.starts_with("~") {
        path_iter.next()?;
        expanded_path = expanded_path.join(dirs_next::home_dir()?);
    }
    for path in path_iter {
        let path = path.to_str()?;
        expanded_path = if cfg!(unix) && path.starts_with('$') {
            expanded_path.join(std::env::var(path.strip_prefix('$')?).unwrap_or_default())
        } else if cfg!(windows) && path.starts_with('%') && path.ends_with('%') {
            expanded_path
                .join(std::env::var(path.strip_prefix('%')?.strip_suffix('%')?).unwrap_or_default())
        } else {
            expanded_path.join(path)
        }
    }
    Some(expanded_path)
}
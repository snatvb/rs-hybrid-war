use bevy_egui::egui;
use derive_more::{Deref, DerefMut};

#[derive(Clone, Debug, PartialEq)]
pub enum LogKind {
    Log,
    Info,
    Debug,
    Error,
    Warning,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsoleLine {
    pub message: String,
    pub kind: LogKind,
}

impl ConsoleLine {
    pub fn new<T: ToString>(message: T, kind: LogKind) -> Self {
        Self {
            message: message.to_string(),
            kind,
        }
    }

    pub(crate) fn get_color(&self) -> egui::Color32 {
        use egui::Color32;
        use LogKind::*;

        match &self.kind {
            Log => Color32::WHITE,
            Info => Color32::GREEN,
            Debug => Color32::GRAY,
            Warning => Color32::YELLOW,
            Error => Color32::LIGHT_RED,
        }
    }
}

#[derive(Default)]
pub struct UiState {
    pub input_value: String,
    pub lines: Vec<ConsoleLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleCommandEvent {
    pub raw: String,
    pub cmd: String,
    pub args: Vec<String>,
}

impl From<&ConsoleLine> for ConsoleCommandEvent {
    fn from(console_line: &ConsoleLine) -> Self {
        ConsoleCommandEvent::from(console_line.message.as_str())
    }
}

impl From<&str> for ConsoleCommandEvent {
    fn from(string: &str) -> Self {
        let splitted: Vec<&str> = string.split(' ').collect();
        let cmd = splitted.first().map(|s| (*s).to_owned()).unwrap_or_else(|| "".to_owned());
        Self {
            cmd,
            raw: string.to_owned(),
            args: splitted
                .iter()
                .skip(1)
                .map(|s| s.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.to_owned())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct ConsoleLogEvent(pub ConsoleLine);

impl ConsoleLogEvent {
    pub fn new<T: ToString>(message: T, kind: LogKind) -> Self {
        Self(ConsoleLine::new(message, kind))
    }
}

impl From<&ConsoleLine> for ConsoleLogEvent {
    fn from(console_line: &ConsoleLine) -> Self {
        Self(console_line.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::ConsoleCommandEvent;

    #[test]
    fn command_from_string() {
        let str_command = "set_scale 1.0";
        let command = ConsoleCommandEvent::from(str_command);
        assert_eq!(command.raw, str_command);
        assert_eq!(command.args, vec!["1.0"]);
        assert_eq!(command.cmd, "set_scale");
    }
}

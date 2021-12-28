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

#[derive(Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct CommandSentEvent(pub String);

impl From<&ConsoleLine> for CommandSentEvent {
    fn from(console_line: &ConsoleLine) -> Self {
        Self(console_line.message.clone())
    }
}

#[derive(Debug, Clone, PartialEq, Deref, DerefMut)]
pub struct WriteLineEvent(pub ConsoleLine);

impl WriteLineEvent {
    pub fn new<T: ToString>(message: T, kind: LogKind) -> Self {
        Self(ConsoleLine::new(message, kind))
    }
}

impl From<&ConsoleLine> for WriteLineEvent {
    fn from(console_line: &ConsoleLine) -> Self {
        Self(console_line.clone())
    }
}

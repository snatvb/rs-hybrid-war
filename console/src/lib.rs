use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use derive_more::{DerefMut, Deref};

pub struct ConsolePlugin;

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
struct UiState {
    pub input_value: String,
    pub lines: Vec<ConsoleLine>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandSentEvent(String)

impl From<&ConsoleLine> for CommandSentEvent {
    fn from(console_line: &ConsoleLine) -> Self {
        Self(console_line.clone())
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

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_event::<WriteLineEvent>();
        app.add_event::<CommandSentEvent>();
        app.init_resource::<UiState>();
        app.add_system(ui_example.system());
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn ui_example(egui_ctx: Res<EguiContext>, mut ui_state: ResMut<UiState>, mut events: EventWriter<CommandSentEvent>) {
    use LogKind::*;

    egui::Window::new("Console").show(egui_ctx.ctx(), |ui| {
        ui.set_width(500.);
        egui::ScrollArea::vertical()
            .max_height(250.)
            .max_width(500.)
            .stick_to_bottom()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for log_line in ui_state.lines.iter() {
                    ui.add(
                        egui::Label::new(log_line.message.clone()).text_color(log_line.get_color()),
                    );
                }
            });
        ui.allocate_space(egui::Vec2::new(1., 5.));
        ui.vertical_centered_justified(|ui| {
            let value = ui_state.input_value.clone();
            let response = ui.text_edit_singleline(&mut ui_state.input_value);
            if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                let console_line = ConsoleLine::new(value, Debug);
                events.send(CommandSentEvent::from(&console_line));
                ui_state.lines.push(console_line);
                ui_state.input_value = "".into();
                response.request_focus();
            }
        });
    });
}

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod structs;
pub use structs::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_event::<WriteLineEvent>();
        app.add_event::<CommandSentEvent>();
        app.init_resource::<UiState>();
        app.add_system(event_to_ui.system().label("event_to_ui"));
        app.add_system(ui.system().label("ui").after("event_to_ui"));
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn event_to_ui(mut ui_state: ResMut<UiState>, mut events: EventReader<WriteLineEvent>) {
    for event in events.iter() {
        ui_state.lines.push(event.0.clone());
    }
}

fn ui(
    egui_ctx: Res<EguiContext>,
    mut ui_state: ResMut<UiState>,
    mut events: EventWriter<CommandSentEvent>,
) {
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

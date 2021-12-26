use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

const FONT: &str = "fonts/Source_Sans_Pro/SourceSansPro-Regular.ttf";

#[derive(Component)]
pub struct FpsText;
#[derive(Component)]
pub struct MsText;
#[derive(Component)]
pub struct Diagnostic;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_bundle = TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            margin: Rect {
                left: Val::Px(10.0),
                top: Val::Px(5.0),
                right: Val::Px(10.0),
                bottom: Val::Px(0.0),
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 22.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 22.0,
                        color: Color::WHITE,
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    };
    commands
        .spawn_bundle(text_bundle.clone())
        .insert(Diagnostic)
        .insert(FpsText);
    commands
        .spawn_bundle(text_bundle)
        .insert(Diagnostic)
        .insert(MsText);
}

pub fn fps_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, (With<FpsText>, With<Diagnostic>)>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = "FPS: ".to_string();
                text.sections[1].value = format!("{:.2}", average);
                let percent = average as f32 / 60.0;
                text.sections[1].style.color.set_r(1.0 - percent);
                text.sections[1].style.color.set_g(percent);
                text.sections[1].style.color.set_b(0.1);
            }
        }
    }
}

pub fn ms_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, (With<MsText>, With<Diagnostic>)>,
) {
    for mut text in query.iter_mut() {
        if let Some(frame_time) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(average) = frame_time.average() {
                let ms = (average * 1000.0) as f32;
                text.sections[0].value = "MS: ".to_string();
                text.sections[1].value = format!("{:.2}", ms);
                let percent = (ms - 16.0) / 16.0;
                text.sections[1].style.color.set_r(percent);
                text.sections[1].style.color.set_g(1.0 - percent);
                text.sections[1].style.color.set_b(0.1);
            }
        }
    }
}
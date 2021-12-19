use bevy::{prelude::*, window::WindowResized};

#[derive(Default)]
pub struct LastSize {
    pub width: f32,
    pub height: f32,
}

pub fn resize_watch(
    mut windows: ResMut<Windows>,
    mut window_resized_events: EventWriter<WindowResized>,
    window_descriptor: Res<WindowDescriptor>,
    mut last_size: ResMut<LastSize>,
) {
    let global_window = web_sys::window().expect("no global `window` exists");
    let width: f32 = (global_window.inner_width().unwrap().as_f64().unwrap() as f32).clamp(
        window_descriptor.resize_constraints.min_width,
        window_descriptor.resize_constraints.max_width,
    );
    let height: f32 = (global_window.inner_height().unwrap().as_f64().unwrap() as f32).clamp(
        window_descriptor.resize_constraints.min_height,
        window_descriptor.resize_constraints.max_height,
    );

    if let Some(window) = windows.get_primary_mut() {
        if width != last_size.width || height != last_size.height {
            *last_size = LastSize { width, height };

            let p_width = width * window.scale_factor() as f32;
            let p_height = height * window.scale_factor() as f32;
            window.update_actual_size_from_backend(p_width as u32, p_height as u32);
            window_resized_events.send(WindowResized {
                id: window.id(),
                height: height,
                width: width,
            });

            let document = global_window.document().expect("Can't get document");
            if let Ok(Some(canvas)) = document.query_selector("canvas[alt=\"bevy\"]") {
                canvas.set_attribute(
                    "width",
                    &(width * global_window.device_pixel_ratio() as f32).to_string(),
                );
                canvas.set_attribute(
                    "height",
                    &(height * global_window.device_pixel_ratio() as f32).to_string(),
                );
                canvas.set_attribute(
                    "style",
                    &format!("width: {}px; height: {}px;", width, height),
                );
            }
        }
    }
}

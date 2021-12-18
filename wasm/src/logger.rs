use bevy_webgl2::renderer::js_sys::Date;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    pub fn log_js(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = warn)]
    pub fn warn_js(s: &str);
}

pub fn format_output(s: String) -> String {
    let date: String = Date::new_0().to_iso_string().into();
    format!("[{}] RS: {}", date, s)
}

#[macro_export]
macro_rules! log {
  ($($t:tt)*) => ($crate::logger::log_js(&$crate::logger::format_output(format_args!($($t)*).to_string())))
}

#[macro_export]
macro_rules! warning {
  ($($t:tt)*) => ($crate::logger::warn_js(&$crate::logger::format_output(format_args!($($t)*).to_string())))
}

pub(crate) use log;
pub(crate) use warning;

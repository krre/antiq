use web_sys::console;

pub fn log(str: &str) {
    console::log_1(&str.into());
}

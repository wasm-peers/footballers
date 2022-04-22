use web_sys::Window;

pub fn global_window() -> Window {
    web_sys::window().expect("there was no window global object!")
}

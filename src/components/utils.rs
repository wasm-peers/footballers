use crate::utils::global_window;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, UrlSearchParams};

pub fn get_query_params() -> UrlSearchParams {
    let search = global_window().location().search().unwrap();
    UrlSearchParams::new_with_str(&search).unwrap()
}

pub fn get_input(id: &str) -> HtmlInputElement {
    global_window()
        .document()
        .expect("document node is missing")
        .get_element_by_id(id)
        .expect("could not find input element by id")
        .dyn_into::<HtmlInputElement>()
        .expect("element is not an input")
}

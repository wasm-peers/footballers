use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, UrlSearchParams};

pub fn get_query_params() -> UrlSearchParams {
    let search = web_sys::window().unwrap().location().search().unwrap();
    UrlSearchParams::new_with_str(&search).unwrap()
}

pub fn get_input(id: &str) -> HtmlInputElement {
    web_sys::window()
        .unwrap()
        .document()
        .expect("document node is missing")
        .get_element_by_id(id)
        .expect("could not find input element by id")
        .dyn_into::<HtmlInputElement>()
        .expect("element is not an input")
}

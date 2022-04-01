use web_sys::{HtmlInputElement, HtmlTextAreaElement, UrlSearchParams};
use wasm_bindgen::JsCast;

pub fn get_query_params() -> UrlSearchParams {
    let search = web_sys::window().unwrap().location().search().unwrap();
    UrlSearchParams::new_with_str(&search).unwrap()
}

pub fn get_text_area(id: &str) -> HtmlTextAreaElement {
    web_sys::window()
        .unwrap()
        .document()
        .expect("document node is missing")
        .get_element_by_id(id)
        .expect("could not find textarea element by id")
        .dyn_into::<HtmlTextAreaElement>()
        .expect("element is not a textarea")
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

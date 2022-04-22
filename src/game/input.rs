use crate::utils::global_window;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct PlayerInput {
    pub(crate) up: bool,
    pub(crate) down: bool,
    pub(crate) left: bool,
    pub(crate) right: bool,
    pub(crate) shoot: bool,
}

pub(crate) fn local_player_input() -> Rc<RefCell<PlayerInput>> {
    let keys_pressed = Rc::new(RefCell::new(PlayerInput::default()));
    let document = global_window().document().unwrap();
    {
        let keys_pressed = keys_pressed.clone();
        let keydown_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = event.key();
            if key.as_str() == "Spacebar" || key.as_str() == " " {
                keys_pressed.borrow_mut().shoot = true;
            }
            match key.as_str() {
                "w" | "ArrowUp" => {
                    keys_pressed.borrow_mut().up = true;
                }
                "a" | "ArrowLeft" => {
                    keys_pressed.borrow_mut().left = true;
                }
                "s" | "ArrowDown" => {
                    keys_pressed.borrow_mut().down = true;
                }
                "d" | "ArrowRight" => {
                    keys_pressed.borrow_mut().right = true;
                }
                _ => {}
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        document
            .add_event_listener_with_callback("keydown", keydown_listener.as_ref().unchecked_ref())
            .unwrap();
        keydown_listener.forget();
    }
    {
        let keys_pressed = keys_pressed.clone();
        let keyup_listener = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            let key = event.key();
            if key.as_str() == "Spacebar" || key.as_str() == " " {
                keys_pressed.borrow_mut().shoot = false;
            }
            match key.as_str() {
                "w" | "ArrowUp" => {
                    keys_pressed.borrow_mut().up = false;
                }
                "a" | "ArrowLeft" => {
                    keys_pressed.borrow_mut().left = false;
                }
                "s" | "ArrowDown" => {
                    keys_pressed.borrow_mut().down = false;
                }
                "d" | "ArrowRight" => {
                    keys_pressed.borrow_mut().right = false;
                }
                _ => {}
            }
        }) as Box<dyn FnMut(KeyboardEvent)>);
        document
            .add_event_listener_with_callback("keyup", keyup_listener.as_ref().unchecked_ref())
            .unwrap();
        keyup_listener.forget();
    }
    keys_pressed
}

use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = EventTarget, extends = Object, js_name = Window, typescript_type = "Window")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Window;
    #[wasm_bindgen (structural, method, getter, js_class = "Window", js_name = window)]
    pub fn window(this: &Window) -> Window;
    #[derive(Debug, Clone, PartialEq)]
    pub type Solana;
    #[wasm_bindgen (structural, method, getter, js_class = "Window", js_name = solana)]
    pub fn solana(this: &Window) -> Option<Solana>;
}

pub fn window() -> Option<Window> {
    use wasm_bindgen::JsCast;

    js_sys::global().dyn_into::<Window>().ok()
}

pub mod components;
pub mod contexts;
pub mod hooks;
pub mod pages;
pub mod views;
pub mod wallets;

// #[wasm_bindgen(start)]
// pub fn bootstrap() -> Result<(), JsValue> {
//     // Use `web_sys`'s global `window` function to get a handle on the global
//     // window object.
//     let window = web_sys::window().expect("no global `window` exists");
//     let document = window.document().expect("should have a document on window");
//     let body = document.body().expect("document should have a body");

//     // Manufacture the element we're gonna append
//     let val = document.create_element("p")?;
//     val.set_inner_html("Hello from Rust!");

//     body.append_child(&val)?;

//     // TODO: execute main.

//     Ok(())
// }

#[derive(PartialEq)]
pub struct Package<'a> {
    pub version: &'a str,
}

#[derive(PartialEq)]
pub struct Wallet {
    pub balance: usize,
}

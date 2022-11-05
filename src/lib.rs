use dioxus::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub mod components;
pub mod views;
pub mod wallets;

use components::{app_bar::AppBar, footer::Footer};
use views::home::HomeView;

#[wasm_bindgen(start)]
pub fn bootstrap() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    // TODO: execute main.

    Ok(())
}

#[derive(PartialEq)]
pub struct Package<'a> {
    pub version: &'a str,
}

#[derive(PartialEq)]
pub struct Wallet {
    pub balance: usize,
}

static STYLES: &'static str = include_str!("../output.css");

#[wasm_bindgen]
pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let window = wallets::window().unwrap();
    let solana = window.solana();
    log::info!("Solana {solana:?}");
    if let Some(s) = solana {
        log::info!(
            "solana connected {}, is_phantom {}, is_brave_wallet {}",
            s.is_connected(),
            s.is_phantom(),
            s.is_brave_wallet(),
        );
    }
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let pkg = Package { version: "0.1.0" };
    let wallet = Wallet { balance: 0 };
    cx.render(rsx! {
        style {
            vec![STYLES]
        }
        div {
            class: "flex flex-col h-screen",
            AppBar {}
            HomeView {
                pkg: pkg,
                wallet: wallet,
            }
            Footer {}
        }
    })
}

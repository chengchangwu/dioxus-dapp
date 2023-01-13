#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_dapp::{
    components::{AppBar, ContentContainer, Footer},
    contexts::NetworkConfiguration,
    pages::{Basics, Home},
    wallets,
};
use dioxus_router::{Route, Router};

static STYLES: &'static str = include_str!("../output.css");

fn main() {
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
    dioxus_web::launch(app);
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || NetworkConfiguration("devnet".to_string()));
    cx.render(rsx! {
        style {
            STYLES
        }
        Router {
            div {
                class: "flex flex-col h-screen",
                AppBar {}
                ContentContainer {
                    Route {
                        to: "/",
                        self::home{}
                    }
                    Route {
                        to: "/basics",
                        self::basics {}
                    }
                }
                Footer {}
            }
        }
    })
}

fn home(cx: Scope) -> Element {
    cx.render(rsx! {
        Home {}
    })
}

fn basics(cx: Scope) -> Element {
    cx.render(rsx! {
        Basics {}
    })
}

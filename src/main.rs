#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_dapp::{
    components::{AppBar, ContentContainer, Footer},
    pages::Basics,
    views::home::HomeView,
    Package, Wallet,
};
use dioxus_router::{Route, Router};

static STYLES: &'static str = include_str!("../output.css");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // let window = wallets::window().unwrap();
    // let solana = window.solana();
    // log::info!("Solana {solana:?}");
    // if let Some(s) = solana {
    //     log::info!(
    //         "solana connected {}, is_phantom {}, is_brave_wallet {}",
    //         s.is_connected(),
    //         s.is_phantom(),
    //         s.is_brave_wallet(),
    //     );
    // }
    dioxus_web::launch(app);
}

// create a component that renders a div with the text "Hello, world!"
fn app(cx: Scope) -> Element {
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
                        self::homepage {}
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

fn homepage(cx: Scope) -> Element {
    let pkg = Package { version: "0.1.0" };
    let wallet = Wallet { balance: 0 };
    cx.render(rsx! {
        HomeView {
            pkg: pkg,
            wallet: wallet,
        }
    })
}

fn basics(cx: Scope) -> Element {
    cx.render(rsx! {
        Basics {}
    })
}

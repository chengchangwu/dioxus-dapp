use dioxus::prelude::*;
use dioxus_dapp::{
    components::{app_bar::AppBar, footer::Footer},
    views::home::HomeView,
    wallets, Package, Wallet,
};

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

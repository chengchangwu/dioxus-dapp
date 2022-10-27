use dioxus::prelude::*;
use dioxus_dapp::{home::HomeView, Package, Wallet};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let pkg = Package { version: "0.1.0" };
    let wallet = Wallet { balance: 0 };
    cx.render(rsx! {
        HomeView{
            pkg: pkg,
            wallet: wallet,
        }
    })
}

use dioxus::prelude::*;

use crate::wallets::ui::Button;

#[derive(Props)]
pub struct WalletModalButtonProps<'a> {
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn WalletModalButton<'a>(cx: Scope<'a, WalletModalButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        Button {
            &cx.props.children
        }
    })
}

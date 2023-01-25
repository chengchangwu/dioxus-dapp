use crate::wallets::ui::{Button, ButtonProps};
use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn WalletModalButton<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        Button {
            start_icon: cx.props.start_icon.as_ref().map(|icon| icon.clone()).unwrap_or(None),
            end_icon: cx.props.end_icon.as_ref().map(|icon| icon.clone()).unwrap_or(None),
            &cx.props.children
        }
    })
}

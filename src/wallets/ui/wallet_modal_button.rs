use dioxus::prelude::*;

use crate::wallets::ui::Button;

#[derive(Props)]
pub struct WalletModalButtonProps<'a> {
    #[props(optional)]
    start_icon: Option<Element<'a>>,
    #[props(optional)]
    end_icon: Option<Element<'a>>,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn WalletModalButton<'a>(cx: Scope<'a, WalletModalButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        Button {
            start_icon: cx.props.start_icon.as_ref().map(|icon| icon.clone()).unwrap_or(None),
            end_icon: cx.props.end_icon.as_ref().map(|icon| icon.clone()).unwrap_or(None),
            &cx.props.children
        }
    })
}

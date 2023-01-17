use dioxus::prelude::*;

#[derive(Props)]
pub struct ButtonProps<'a> {
    #[props(default = "")]
    class: &'a str,
    #[props(default = false)]
    disabled: bool,
    #[props(default = None)]
    end_icon: Element<'a>,
    // onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
    #[props(default = None)]
    start_icon: Element<'a>,
    // style?: CSSProperties;
    // tabIndex?: number;
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    cx.render(rsx! {
        button {
            class: "wallet-adapter-button {cx.props.class}",
            disabled: cx.props.disabled,
            // style={props.style}
            // onClick={props.onClick}
            // tabIndex={props.tabIndex || 0}
            r#type: "button",
            // TODO
            // &cx.props.start_icon.map(|icon| rsx!{
            //     i {
            //         class: "wallet-adapter-button-start-icon",
            //         icon
            //     }
            // })
            &cx.props.children
            // &cx.props.end_icon.map(|icon| rsx!{
            //     i {
            //         class: "wallet-adapter-button-end-icon",
            //         icon
            //     }
            // })
        }
    })
}

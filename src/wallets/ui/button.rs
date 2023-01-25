use dioxus::prelude::*;

#[derive(Props, Clone)]
pub struct ButtonProps<'a> {
    #[props(default)]
    pub class: &'a str,
    #[props(default)]
    pub disabled: bool,
    #[props(optional)]
    pub end_icon: Option<Element<'a>>,
    // onClick?: (e: MouseEvent<HTMLButtonElement>) => void;
    #[props(optional)]
    pub start_icon: Option<Element<'a>>,
    // style?: CSSProperties;
    // tabIndex?: number;
    pub children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, ButtonProps<'a>>) -> Element<'a> {
    let start_icon = match &cx.props.start_icon {
        Some(Some(node)) => {
            rsx! {
                i {
                    class: "wallet-adapter-button-start-icon",
                    node
                }
            }
        }
        Some(None) => {
            rsx! { Option::<VNode>::None }
        }
        None => {
            rsx! { Option::<VNode>::None }
        }
    };
    let end_icon = match &cx.props.end_icon {
        Some(Some(node)) => {
            rsx! {
                i {
                    class: "wallet-adapter-button-end-icon",
                    node
                }
            }
        }
        Some(None) => {
            rsx! { Option::<VNode>::None }
        }
        None => {
            rsx! { Option::<VNode>::None }
        }
    };
    cx.render(rsx! {
        button {
            class: "wallet-adapter-button {cx.props.class}",
            disabled: cx.props.disabled,
            // style={props.style}
            // onClick={props.onClick}
            // tabIndex={props.tabIndex || 0}
            r#type: "button",
            start_icon,
            &cx.props.children,
            end_icon
        }
    })
}
use dioxus::{hooks, prelude::*};
use serde::{de::DeserializeOwned, Serialize};

use crate::wallets;

pub fn use_local_storage<'a, 'b, T: Serialize + DeserializeOwned + core::fmt::Debug + PartialEq>(
    cx: &'a ScopeState,
    key: &'static str,
    default: T,
) -> &'a hooks::UseState<T> {
    let state: &'a UseState<T> = use_state(cx, || {
        let storage = wallets::window().unwrap().local_storage().unwrap().unwrap();
        match storage.get_item(key).unwrap() {
            Some(v) => match ron::from_str(&v) {
                Ok(s) => {
                    log::debug!("got {s:#?}");
                    s
                }
                Err(e) => {
                    log::debug!("Error: {e}");
                    default
                }
            },
            None => {
                log::debug!("No key");
                default
            }
        }
    });
    let value = state.current();
    let is_first_render = use_ref(cx, || true);
    let is_first_render = is_first_render.clone();
    use_effect(cx, (&key, &value), |(key, value)| async move {
        if *is_first_render.read() == true {
            *is_first_render.write() = false;
            log::debug!("first render, key: {key}, value: {value:#?}");
            return;
        }
        let storage = wallets::window().unwrap().local_storage().unwrap().unwrap();
        log::debug!("set item ({key}, {value:#?})");
        storage
            .set_item(&key, &ron::to_string(&*value).unwrap())
            .unwrap();
    });
    state
}

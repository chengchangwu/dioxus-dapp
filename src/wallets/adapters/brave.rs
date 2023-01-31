//! https://wallet-docs.brave.com/

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/ts/node_modules/@solana/wallet-adapter-brave/lib/esm/adapter.js")]
extern "C" {
    pub type BraveWalletAdapter;
    #[wasm_bindgen(constructor)]
    pub fn new() -> BraveWalletAdapter;
    // TODO
    // #[wasm_bindgen(method, getter)]
    // fn publicKey(this: &BraveWalletAdapter) -> PublicKey;
    #[wasm_bindgen(method, getter)]
    fn connecting(this: &BraveWalletAdapter) -> bool;
    #[wasm_bindgen(method, getter)]
    fn connected(this: &BraveWalletAdapter) -> bool;
    #[wasm_bindgen(method, getter)]
    fn readyState(this: &BraveWalletAdapter) -> JsValue;
    #[wasm_bindgen(catch, method)]
    async fn connect(this: &BraveWalletAdapter) -> Result<(), JsValue>;
}

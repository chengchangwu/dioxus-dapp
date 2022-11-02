//! https://wallet-docs.brave.com/

use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/node_modules/@solana/wallet-adapter-brave/lib/cjs/adapter.js")]
extern "C" {
    type BraveWalletAdapter;
    #[wasm_bindgen(constructor)]
    fn new() -> BraveWalletAdapter;
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

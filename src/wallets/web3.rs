use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/node_modules/@solana/web3.js/lib/index.browser.cjs.js")]
extern "C" {
    pub type PublicKeyInitData;
    pub type PublicKey;
    #[wasm_bindgen(constructor)]
    pub fn new(value: &PublicKeyInitData) -> PublicKey;
    #[wasm_bindgen(method, getter, js_name = "toBase58")]
    pub fn to_base_58(this: &PublicKey) -> String;
    #[wasm_bindgen(method, getter, js_name = "toBytes")]
    pub fn to_bytes(this: &PublicKey) -> Uint8Array;
}

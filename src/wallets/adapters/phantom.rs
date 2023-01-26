use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/node_modules/@solana/wallet-adapter-phantom/lib/cjs/adapter.js")]
extern "C" {
    type PhantomWalletAdapter;
    #[wasm_bindgen(constructor)]
    fn new() -> PhantomWalletAdapter;
    // TODO
    // #[wasm_bindgen(method, getter)]
    // fn publicKey(this: &PhantomWalletAdapter) -> PublicKey;
    #[wasm_bindgen(method, getter)]
    fn connecting(this: &PhantomWalletAdapter) -> bool;
    #[wasm_bindgen(method, getter)]
    fn connected(this: &PhantomWalletAdapter) -> bool;
    #[wasm_bindgen(method, getter)]
    fn readyState(this: &PhantomWalletAdapter) -> JsValue;
    #[wasm_bindgen(catch, method)]
    async fn connect(this: &PhantomWalletAdapter) -> Result<(), JsValue>;
}

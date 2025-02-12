#[cfg(target_family="wasm")]
#[ctor::ctor]
unsafe fn hello() {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    log("hello!");
}

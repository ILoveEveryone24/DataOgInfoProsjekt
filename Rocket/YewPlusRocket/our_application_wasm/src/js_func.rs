use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/js/Another.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn name() -> String;
}
                
// #[wasm_bindgen(start)]
// pub fn run() {
//     log!(&format!("Hello from {}!", name())); // should output "Hello from Rust!"
// }

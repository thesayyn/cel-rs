use program::{Context, Program};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(message: String);
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);
}

#[wasm_bindgen]
pub fn execute(source: &str) -> bool {
    log(format!("parsing {}", source));
    match Program::new(source) {
        Ok(p) => p.execute(Context::default()),
        Err(err) => {
            error(format!("parse error {}", err));
            false
        },
    }
}

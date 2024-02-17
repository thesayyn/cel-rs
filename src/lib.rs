use program::{Context, Program};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(message: String);
    #[wasm_bindgen(js_namespace = console)]
    fn log(message: String);
}

fn eval(source: &str) -> bool {
    match Program::new(source) {
        Ok(p) => p.execute(Context::default()),
        Err(err) => {
            error(format!("cel-rs: parse error {}", err));
            false
        },
    }
}


#[wasm_bindgen]
pub fn execute(source: &str) -> Result<bool, JsError> {
    log(format!("cel-rs: parsing {}", source));
    let res = std::panic::catch_unwind(|| eval(source));
    match res {
        Ok(b) => Ok(b),
        Err(e) => {
            log(format!("{:?}", e));
            Err(JsError::new(format!("{:?}", e).as_str()))
        } 
    }
}

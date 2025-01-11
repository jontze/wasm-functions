use wasm_function_sdk::scheduled::{export, Function};

struct Component;

impl Function for Component {
    fn run_job() -> Result<(), ()> {
        Ok(())
    }
}

export!(Component with_types_in wasm_function_sdk::scheduled);

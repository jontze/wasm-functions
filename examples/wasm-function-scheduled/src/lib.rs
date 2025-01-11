use wasm_function_sdk::blocking::scheduled::{export, Function};

struct Component;

impl Function for Component {
    fn run_job() -> Result<(), ()> {
        Ok(())
    }
}

export!(Component);

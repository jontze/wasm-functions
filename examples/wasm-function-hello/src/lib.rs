use wasm_function_sdk::scheduled::{export, Function};

struct Component;

impl Function for Component {
    /// Say hello!
    fn run_job() -> Result<(), ()> {
        println!("Hello, World!");
        Ok(())
    }
}

export!(Component with_types_in wasm_function_sdk::scheduled);

use wasm_function_sdk::blocking::scheduled::{export, Function};

struct Component;

impl Function for Component {
    /// Say hello!
    fn run_job() -> Result<(), ()> {
        println!("Hello, World!");
        Ok(())
    }
}

export!(Component);

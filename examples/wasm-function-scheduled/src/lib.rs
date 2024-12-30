#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn run_job() -> Result<(), ()> {
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);

#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn handle_request(req: bindings::Request) -> Result<bindings::Response, ()> {
        todo!()
    }
}

bindings::export!(Component with_types_in bindings);

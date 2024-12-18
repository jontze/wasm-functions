#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn handle_request(_req: bindings::Request) -> Result<bindings::Response, ()> {
        let res = bindings::Response {
            headers: vec![],
            status_code: 200,
            body: "Module executed".as_bytes().to_vec(),
        };
        Ok(res)
    }
}

bindings::export!(Component with_types_in bindings);

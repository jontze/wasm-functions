#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn handle_request(_: bindings::Request) -> Result<bindings::Response, ()> {
        Ok(bindings::Response {
            status_code: 200,
            headers: vec![bindings::Header {
                name: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }],
            body: include_bytes!("./index.html").to_vec(),
        })
    }
}

bindings::export!(Component with_types_in bindings);

use wasm_function_sdk::http::{export, Function, Header, Request, Response};

struct Component;

impl Function for Component {
    fn handle_request(_: Request) -> Result<Response, ()> {
        Ok(Response {
            status_code: 200,
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "text/html".to_string(),
            }],
            body: include_bytes!("./index.html").to_vec(),
        })
    }
}

export!(Component with_types_in wasm_function_sdk::http);

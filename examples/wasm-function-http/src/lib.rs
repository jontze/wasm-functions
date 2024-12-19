#[allow(warnings)]
mod bindings;

use std::collections::HashMap;

use bindings::Guest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct JsonResponse {
    args: HashMap<String, String>,
}

struct Component;

impl Guest for Component {
    fn handle_request(_req: bindings::Request) -> Result<bindings::Response, ()> {
        let client = waki::Client::new();
        let response = client
            .get("https://httpbin.org/get?a=b")
            .headers([("Content-Type", "application/json"), ("Accept", "*/*")])
            .send()
            .expect("Failed to send request")
            .json::<JsonResponse>()
            .expect("Failed to parse response");

        // Do something with the response
        //...

        // Return something
        let res = bindings::Response {
            headers: vec![bindings::Header {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            }],
            status_code: 200,
            body: serde_json::to_vec(&response).expect("Failed to serialize response"),
        };
        Ok(res)
    }
}

bindings::export!(Component with_types_in bindings);

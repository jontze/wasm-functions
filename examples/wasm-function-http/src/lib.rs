use serde::{Deserialize, Serialize};
use wasm_function_sdk::http::{export, Function, Header, Request, Response};

use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
struct JsonResponse {
    args: HashMap<String, String>,
}

#[derive(Serialize)]
struct FunctionResponse {
    http_response: JsonResponse,
    var_envs: HashMap<String, String>,
}

struct Component;

impl Function for Component {
    fn handle_request(_req: Request) -> Result<Response, ()> {
        let client = waki::Client::new();
        let http_response = client
            .get("https://httpbin.org/get?a=b")
            .headers([("Content-Type", "application/json"), ("Accept", "*/*")])
            .send()
            .expect("Failed to send request")
            .json::<JsonResponse>()
            .expect("Failed to parse response");

        // Do something with the response
        //...

        // Load all envs into a hashmap if thet are prefixed with VAR_
        let var_envs = std::env::vars()
            .filter(|(k, _)| k.starts_with("VAR_"))
            .collect::<HashMap<String, String>>();

        // Return something
        let res = Response {
            headers: vec![Header {
                name: "Content-Type".to_string(),
                value: "application/json".to_string(),
            }],
            status_code: 200,
            body: serde_json::to_vec(&FunctionResponse {
                http_response,
                var_envs,
            })
            .expect("Failed to serialize response"),
        };
        Ok(res)
    }
}

export!(Component with_types_in wasm_function_sdk::http);

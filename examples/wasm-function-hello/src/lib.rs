#[allow(warnings)]
mod bindings;

use bindings::Guest;

struct Component;

impl Guest for Component {
    fn handle_request(_req: bindings::Request) -> Result<bindings::Response, ()> {
        let mut response = bindings::Response {
            status_code: 200,
            headers: vec![bindings::Header {
                name: "Content-Type".to_string(),
                value: "text/plain".to_string(),
            }],
            body: vec![],
        };

        let envs: Vec<(String, String)> = std::env::vars().collect();

        let key_value_string = envs
            .iter()
            .map(|(k, v)| format!("{}: {}\n", k, v))
            .collect::<String>();

        response.body = key_value_string.as_bytes().to_vec();
        Ok(response)
    }
}

bindings::export!(Component with_types_in bindings);

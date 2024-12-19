#[allow(warnings)]
mod bindings;

use bindings::Guest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct CommitDetails {
    sha: String,
    commit: Commit,
}

#[derive(Deserialize, Serialize)]
struct Commit {
    message: String,
}

struct Component;

impl Guest for Component {
    fn handle_request(_req: bindings::Request) -> Result<bindings::Response, ()> {
        let client = waki::Client::new();
        let response = client
            .get("https://api.github.com/repos/jontze/wasm-functions/commits")
            .send()
            .map_err(|_| ())?
            .json::<CommitDetails>()
            .map_err(|_| ())?;

        // Do something with the response
        //...

        // Return something
        let res = bindings::Response {
            headers: vec![],
            status_code: 200,
            body: serde_json::to_vec(&response).map_err(|_| ())?,
        };
        Ok(res)
    }
}

bindings::export!(Component with_types_in bindings);

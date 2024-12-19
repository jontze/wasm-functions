## WASM Function with HTTP Client

This example crate showcases how to create a WASM Component function that uses an http client to do request to external domains and return json data.

## Build

```sh
cargo build --release --target=wasm32-wasip2
```

## Deploy

Deploy the built wasm file with a manifest to the runtime.

```sh
curl -X POST "http://localhost:3000/api/deploy" \
     -F "file1=@./manifest.toml" \
     -F "file2=@../../target/wasm32-wasip2/release/wasm_function_http.wasm"
```

```ps6
Invoke-RestMethod -Uri "http://localhost:3000/api/deploy" -Method Post -Form @{ "file1" = Get-Item .\manifest.toml; "file2" = Get-Item ..\..\target\wasm32-wasip2\release\wasm_function_html.wasm }
```

## Usae

Call the http trigger on the specified path: [http://localhost:3000/function/html/hello](http://localhost:3000/function/http/client).

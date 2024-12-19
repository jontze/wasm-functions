# HTML WASM Function

This example crate showcases how to create a HTML page that is served via WASM component and can be executed and used on a generic WASM WASI host.

## Build

```sh
cargo build --release --target=wasm32-wasip2
```

## Deploy

Deploy the built wasm file with a manifest to the runtime.

```sh

```

```ps6
Invoke-RestMethod -Uri "http://localhost:3000/api/deploy" -Method Post -Form @{ "file1" = Get-Item .\manifest.toml; "file2" = Get-Item ..\..\target\wasm32-wasip2\release\wasm_function_html.wasm }
```

## Usae

Call the http trigger on the specified path: [http://localhost:3000/function/html/hello](http://localhost:3000/function/html/hello).

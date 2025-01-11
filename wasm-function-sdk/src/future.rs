#[allow(clippy::too_many_arguments)]
#[cfg(feature = "http")]
pub mod http_async {

    wit_bindgen::generate!({
        world: "function-http",
         path: "./wit-http/",
         pub_export_macro: true,
         default_bindings_module: "wasm_function_sdk::future::http",
         async: true,
    });

    pub use self::Guest as Function;
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "scheduled")]
pub mod scheduled_async {

    wit_bindgen::generate!({
        world: "function-scheduled",
        path: "./wit-scheduled/",
        pub_export_macro: true,
        default_bindings_module: "wasm_function_sdk::future::scheduled",
        async: true,
    });

    pub use self::Guest as Function;
}

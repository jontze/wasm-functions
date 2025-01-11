#[allow(clippy::too_many_arguments)]
#[cfg(feature = "http")]
pub mod http {

    wit_bindgen::generate!({
        world: "function-http",
        path: "./wit-http/",
        pub_export_macro: true,
        default_bindings_module: "wasm_function_sdk::blocking::http",
        export_macro_name: "export",
    });

    pub use self::Guest as Function;
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "scheduled")]
pub mod scheduled {

    wit_bindgen::generate!({
        world: "function-scheduled",
        path: "./wit-scheduled/",
        pub_export_macro: true,
        default_bindings_module: "wasm_function_sdk::blocking::scheduled",
        export_macro_name: "export",
    });

    pub use self::Guest as Function;
}

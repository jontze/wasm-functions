#[allow(clippy::too_many_arguments)]
pub mod http {

    wit_bindgen::generate!({
        world: "function-http",
         path: "./wit-http/",
         pub_export_macro: true,
         export_macro_name: "export",
    });

    pub use self::Guest as Function;
}

#[allow(clippy::too_many_arguments)]
pub mod scheduled {

    wit_bindgen::generate!({
        world: "function-scheduled",
        path: "./wit-scheduled/",
        pub_export_macro: true,
        export_macro_name: "export",
    });

    pub use self::Guest as Function;
}

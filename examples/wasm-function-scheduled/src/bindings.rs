#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_run_job_cabi<T: Guest>() -> i32 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let result0 = T::run_job();
    let result1 = match result0 {
        Ok(_) => 0i32,
        Err(_) => 1i32,
    };
    result1
}
pub trait Guest {
    fn run_job() -> Result<(), ()>;
}
#[doc(hidden)]
macro_rules! __export_world_function_scheduled_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "run-job"] unsafe extern "C" fn export_run_job()
        -> i32 { $($path_to_types)*:: _export_run_job_cabi::<$ty > () } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_function_scheduled_cabi;
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_function_scheduled_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_function_scheduled_cabi!($ty
        with_types_in $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_function_scheduled_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:jontze:function-scheduled:function-scheduled:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 212] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07L\x01A\x02\x01A\x03\x01\
j\0\0\x01@\0\0\0\x04\0\x07run-job\x01\x01\x04\0,jontze:function-scheduled/functi\
on-scheduled\x04\0\x0b\x18\x01\0\x12function-scheduled\x03\0\0\0G\x09producers\x01\
\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

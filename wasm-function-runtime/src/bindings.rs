/// Represents a general HTTP header, e.g. ("Content-Type", "application/json")
#[derive(Clone)]
pub struct Header {
    pub name: _rt::String,
    pub value: _rt::String,
}
impl ::core::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Header")
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}
/// Http Methods
#[repr(u8)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
}
impl ::core::fmt::Debug for Method {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
            Method::Get => f.debug_tuple("Method::Get").finish(),
            Method::Post => f.debug_tuple("Method::Post").finish(),
            Method::Put => f.debug_tuple("Method::Put").finish(),
            Method::Delete => f.debug_tuple("Method::Delete").finish(),
            Method::Patch => f.debug_tuple("Method::Patch").finish(),
            Method::Options => f.debug_tuple("Method::Options").finish(),
            Method::Head => f.debug_tuple("Method::Head").finish(),
        }
    }
}
impl Method {
    #[doc(hidden)]
    pub unsafe fn _lift(val: u8) -> Method {
        if !cfg!(debug_assertions) {
            return ::core::mem::transmute(val);
        }
        match val {
            0 => Method::Get,
            1 => Method::Post,
            2 => Method::Put,
            3 => Method::Delete,
            4 => Method::Patch,
            5 => Method::Options,
            6 => Method::Head,
            _ => panic!("invalid enum discriminant"),
        }
    }
}
/// Represents an inbound HTTP request to your serverless function.
#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub path: _rt::String,
    /// Key-value pairs representing the query parameters in the URL
    pub query_params: _rt::Vec<Header>,
    /// Key-value pairs representing the request headers
    pub headers: _rt::Vec<Header>,
    /// Raw request body bytes (could be JSON, form data, etc.)
    pub body: _rt::Vec<u8>,
}
impl ::core::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Request")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("query-params", &self.query_params)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}
#[derive(Clone)]
pub struct Response {
    pub status_code: u16,
    pub headers: _rt::Vec<Header>,
    pub body: _rt::Vec<u8>,
}
impl ::core::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Response")
            .field("status-code", &self.status_code)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_path_cabi<T: Guest>() -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let result0 = T::path();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let vec2 = (result0.into_bytes()).into_boxed_slice();
    let ptr2 = vec2.as_ptr().cast::<u8>();
    let len2 = vec2.len();
    ::core::mem::forget(vec2);
    *ptr1.add(4).cast::<usize>() = len2;
    *ptr1.add(0).cast::<*mut u8>() = ptr2.cast_mut();
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_path<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(0).cast::<*mut u8>();
    let l1 = *arg0.add(4).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_handle_request_cabi<T: Guest>(
    arg0: i32,
    arg1: *mut u8,
    arg2: usize,
    arg3: *mut u8,
    arg4: usize,
    arg5: *mut u8,
    arg6: usize,
    arg7: *mut u8,
    arg8: usize,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg2;
    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
    let base7 = arg3;
    let len7 = arg4;
    let mut result7 = _rt::Vec::with_capacity(len7);
    for i in 0..len7 {
        let base = base7.add(i * 16);
        let e7 = {
            let l1 = *base.add(0).cast::<*mut u8>();
            let l2 = *base.add(4).cast::<usize>();
            let len3 = l2;
            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
            let l4 = *base.add(8).cast::<*mut u8>();
            let l5 = *base.add(12).cast::<usize>();
            let len6 = l5;
            let bytes6 = _rt::Vec::from_raw_parts(l4.cast(), len6, len6);
            Header {
                name: _rt::string_lift(bytes3),
                value: _rt::string_lift(bytes6),
            }
        };
        result7.push(e7);
    }
    _rt::cabi_dealloc(base7, len7 * 16, 4);
    let base14 = arg5;
    let len14 = arg6;
    let mut result14 = _rt::Vec::with_capacity(len14);
    for i in 0..len14 {
        let base = base14.add(i * 16);
        let e14 = {
            let l8 = *base.add(0).cast::<*mut u8>();
            let l9 = *base.add(4).cast::<usize>();
            let len10 = l9;
            let bytes10 = _rt::Vec::from_raw_parts(l8.cast(), len10, len10);
            let l11 = *base.add(8).cast::<*mut u8>();
            let l12 = *base.add(12).cast::<usize>();
            let len13 = l12;
            let bytes13 = _rt::Vec::from_raw_parts(l11.cast(), len13, len13);
            Header {
                name: _rt::string_lift(bytes10),
                value: _rt::string_lift(bytes13),
            }
        };
        result14.push(e14);
    }
    _rt::cabi_dealloc(base14, len14 * 16, 4);
    let len15 = arg8;
    let result16 = T::handle_request(Request {
        method: Method::_lift(arg0 as u8),
        path: _rt::string_lift(bytes0),
        query_params: result7,
        headers: result14,
        body: _rt::Vec::from_raw_parts(arg7.cast(), len15, len15),
    });
    let ptr17 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result16 {
        Ok(e) => {
            *ptr17.add(0).cast::<u8>() = (0i32) as u8;
            let Response {
                status_code: status_code18,
                headers: headers18,
                body: body18,
            } = e;
            *ptr17.add(4).cast::<u16>() = (_rt::as_i32(status_code18)) as u16;
            let vec22 = headers18;
            let len22 = vec22.len();
            let layout22 = _rt::alloc::Layout::from_size_align_unchecked(
                vec22.len() * 16,
                4,
            );
            let result22 = if layout22.size() != 0 {
                let ptr = _rt::alloc::alloc(layout22).cast::<u8>();
                if ptr.is_null() {
                    _rt::alloc::handle_alloc_error(layout22);
                }
                ptr
            } else {
                ::core::ptr::null_mut()
            };
            for (i, e) in vec22.into_iter().enumerate() {
                let base = result22.add(i * 16);
                {
                    let Header { name: name19, value: value19 } = e;
                    let vec20 = (name19.into_bytes()).into_boxed_slice();
                    let ptr20 = vec20.as_ptr().cast::<u8>();
                    let len20 = vec20.len();
                    ::core::mem::forget(vec20);
                    *base.add(4).cast::<usize>() = len20;
                    *base.add(0).cast::<*mut u8>() = ptr20.cast_mut();
                    let vec21 = (value19.into_bytes()).into_boxed_slice();
                    let ptr21 = vec21.as_ptr().cast::<u8>();
                    let len21 = vec21.len();
                    ::core::mem::forget(vec21);
                    *base.add(12).cast::<usize>() = len21;
                    *base.add(8).cast::<*mut u8>() = ptr21.cast_mut();
                }
            }
            *ptr17.add(12).cast::<usize>() = len22;
            *ptr17.add(8).cast::<*mut u8>() = result22;
            let vec23 = (body18).into_boxed_slice();
            let ptr23 = vec23.as_ptr().cast::<u8>();
            let len23 = vec23.len();
            ::core::mem::forget(vec23);
            *ptr17.add(20).cast::<usize>() = len23;
            *ptr17.add(16).cast::<*mut u8>() = ptr23.cast_mut();
        }
        Err(_) => {
            *ptr17.add(0).cast::<u8>() = (1i32) as u8;
        }
    };
    ptr17
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_handle_request<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => {
            let l1 = *arg0.add(8).cast::<*mut u8>();
            let l2 = *arg0.add(12).cast::<usize>();
            let base7 = l1;
            let len7 = l2;
            for i in 0..len7 {
                let base = base7.add(i * 16);
                {
                    let l3 = *base.add(0).cast::<*mut u8>();
                    let l4 = *base.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l3, l4, 1);
                    let l5 = *base.add(8).cast::<*mut u8>();
                    let l6 = *base.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l5, l6, 1);
                }
            }
            _rt::cabi_dealloc(base7, len7 * 16, 4);
            let l8 = *arg0.add(16).cast::<*mut u8>();
            let l9 = *arg0.add(20).cast::<usize>();
            let base10 = l8;
            let len10 = l9;
            _rt::cabi_dealloc(base10, len10 * 1, 1);
        }
        _ => {}
    }
}
pub trait Guest {
    fn path() -> _rt::String;
    fn handle_request(req: Request) -> Result<Response, ()>;
}
#[doc(hidden)]
macro_rules! __export_world_function_http_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "path"] unsafe extern "C" fn export_path() -> *
        mut u8 { $($path_to_types)*:: _export_path_cabi::<$ty > () } #[export_name =
        "cabi_post_path"] unsafe extern "C" fn _post_return_path(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_path::<$ty > (arg0) } #[export_name =
        "handle-request"] unsafe extern "C" fn export_handle_request(arg0 : i32, arg1 : *
        mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize, arg5 : * mut u8, arg6 :
        usize, arg7 : * mut u8, arg8 : usize,) -> * mut u8 { $($path_to_types)*::
        _export_handle_request_cabi::<$ty > (arg0, arg1, arg2, arg3, arg4, arg5, arg6,
        arg7, arg8) } #[export_name = "cabi_post_handle-request"] unsafe extern "C" fn
        _post_return_handle_request(arg0 : * mut u8,) { $($path_to_types)*::
        __post_return_handle_request::<$ty > (arg0) } };
    };
}
#[doc(hidden)]
pub(crate) use __export_world_function_http_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 24]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 24]);
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::alloc;
    extern crate alloc as alloc_crate;
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
macro_rules! __export_function_http_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*:: __export_world_function_http_cabi!($ty with_types_in
        $($path_to_types_root)*);
    };
}
#[doc(inline)]
pub(crate) use __export_function_http_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:jontze:function-http:function-http:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 417] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9d\x02\x01A\x02\x01\
A\x0f\x01r\x02\x04names\x05values\x03\0\x06header\x03\0\0\x01m\x07\x03GET\x04POS\
T\x03PUT\x06DELETE\x05PATCH\x07OPTIONS\x04HEAD\x03\0\x06method\x03\0\x02\x01p\x01\
\x01p}\x01r\x05\x06method\x03\x04paths\x0cquery-params\x04\x07headers\x04\x04bod\
y\x05\x03\0\x07request\x03\0\x06\x01r\x03\x0bstatus-code{\x07headers\x04\x04body\
\x05\x03\0\x08response\x03\0\x08\x01@\0\0s\x04\0\x04path\x01\x0a\x01j\x01\x09\0\x01\
@\x01\x03req\x07\0\x0b\x04\0\x0ehandle-request\x01\x0c\x04\0\"jontze:function-ht\
tp/function-http\x04\0\x0b\x13\x01\0\x0dfunction-http\x03\0\0\0G\x09producers\x01\
\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

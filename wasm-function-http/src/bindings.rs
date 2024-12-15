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
/// Represents an inbound HTTP request to your serverless function.
#[derive(Clone)]
pub struct Request {
    pub method: _rt::String,
    pub path: _rt::String,
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
pub unsafe fn _export_handle_request_cabi<T: Guest>(
    arg0: *mut u8,
    arg1: usize,
    arg2: *mut u8,
    arg3: usize,
    arg4: *mut u8,
    arg5: usize,
    arg6: *mut u8,
    arg7: usize,
) -> *mut u8 {
    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    let len1 = arg3;
    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
    let base8 = arg4;
    let len8 = arg5;
    let mut result8 = _rt::Vec::with_capacity(len8);
    for i in 0..len8 {
        let base = base8.add(i * 16);
        let e8 = {
            let l2 = *base.add(0).cast::<*mut u8>();
            let l3 = *base.add(4).cast::<usize>();
            let len4 = l3;
            let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
            let l5 = *base.add(8).cast::<*mut u8>();
            let l6 = *base.add(12).cast::<usize>();
            let len7 = l6;
            let bytes7 = _rt::Vec::from_raw_parts(l5.cast(), len7, len7);
            Header {
                name: _rt::string_lift(bytes4),
                value: _rt::string_lift(bytes7),
            }
        };
        result8.push(e8);
    }
    _rt::cabi_dealloc(base8, len8 * 16, 4);
    let len9 = arg7;
    let result10 = T::handle_request(Request {
        method: _rt::string_lift(bytes0),
        path: _rt::string_lift(bytes1),
        headers: result8,
        body: _rt::Vec::from_raw_parts(arg6.cast(), len9, len9),
    });
    let ptr11 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    match result10 {
        Ok(e) => {
            *ptr11.add(0).cast::<u8>() = (0i32) as u8;
            let Response {
                status_code: status_code12,
                headers: headers12,
                body: body12,
            } = e;
            *ptr11.add(4).cast::<u16>() = (_rt::as_i32(status_code12)) as u16;
            let vec16 = headers12;
            let len16 = vec16.len();
            let layout16 = _rt::alloc::Layout::from_size_align_unchecked(
                vec16.len() * 16,
                4,
            );
            let result16 = if layout16.size() != 0 {
                let ptr = _rt::alloc::alloc(layout16).cast::<u8>();
                if ptr.is_null() {
                    _rt::alloc::handle_alloc_error(layout16);
                }
                ptr
            } else {
                ::core::ptr::null_mut()
            };
            for (i, e) in vec16.into_iter().enumerate() {
                let base = result16.add(i * 16);
                {
                    let Header { name: name13, value: value13 } = e;
                    let vec14 = (name13.into_bytes()).into_boxed_slice();
                    let ptr14 = vec14.as_ptr().cast::<u8>();
                    let len14 = vec14.len();
                    ::core::mem::forget(vec14);
                    *base.add(4).cast::<usize>() = len14;
                    *base.add(0).cast::<*mut u8>() = ptr14.cast_mut();
                    let vec15 = (value13.into_bytes()).into_boxed_slice();
                    let ptr15 = vec15.as_ptr().cast::<u8>();
                    let len15 = vec15.len();
                    ::core::mem::forget(vec15);
                    *base.add(12).cast::<usize>() = len15;
                    *base.add(8).cast::<*mut u8>() = ptr15.cast_mut();
                }
            }
            *ptr11.add(12).cast::<usize>() = len16;
            *ptr11.add(8).cast::<*mut u8>() = result16;
            let vec17 = (body12).into_boxed_slice();
            let ptr17 = vec17.as_ptr().cast::<u8>();
            let len17 = vec17.len();
            ::core::mem::forget(vec17);
            *ptr11.add(20).cast::<usize>() = len17;
            *ptr11.add(16).cast::<*mut u8>() = ptr17.cast_mut();
        }
        Err(_) => {
            *ptr11.add(0).cast::<u8>() = (1i32) as u8;
        }
    };
    ptr11
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
    fn handle_request(req: Request) -> Result<Response, ()>;
}
#[doc(hidden)]
macro_rules! __export_world_function_http_cabi {
    ($ty:ident with_types_in $($path_to_types:tt)*) => {
        const _ : () = { #[export_name = "handle-request"] unsafe extern "C" fn
        export_handle_request(arg0 : * mut u8, arg1 : usize, arg2 : * mut u8, arg3 :
        usize, arg4 : * mut u8, arg5 : usize, arg6 : * mut u8, arg7 : usize,) -> * mut u8
        { $($path_to_types)*:: _export_handle_request_cabi::<$ty > (arg0, arg1, arg2,
        arg3, arg4, arg5, arg6, arg7) } #[export_name = "cabi_post_handle-request"]
        unsafe extern "C" fn _post_return_handle_request(arg0 : * mut u8,) {
        $($path_to_types)*:: __post_return_handle_request::<$ty > (arg0) } };
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
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
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
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 335] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xcb\x01\x01A\x02\x01\
A\x0b\x01r\x02\x04names\x05values\x03\0\x06header\x03\0\0\x01p\x01\x01p}\x01r\x04\
\x06methods\x04paths\x07headers\x02\x04body\x03\x03\0\x07request\x03\0\x04\x01r\x03\
\x0bstatus-code{\x07headers\x02\x04body\x03\x03\0\x08response\x03\0\x06\x01j\x01\
\x07\0\x01@\x01\x03req\x05\0\x08\x04\0\x0ehandle-request\x01\x09\x04\0\"jontze:f\
unction-http/function-http\x04\0\x0b\x13\x01\0\x0dfunction-http\x03\0\0\0G\x09pr\
oducers\x01\x0cprocessed-by\x02\x0dwit-component\x070.220.0\x10wit-bindgen-rust\x06\
0.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}

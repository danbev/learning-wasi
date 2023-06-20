// Generated by `wit-bindgen` 0.7.0. DO NOT EDIT!
pub trait Component {
  fn something(s: wit_bindgen::rt::string::String,) -> wit_bindgen::rt::string::String;
}

#[doc(hidden)]
pub unsafe fn call_something<T: Component>(arg0: i32,arg1: i32,) -> i32 {
  
  #[allow(unused_imports)]
  use wit_bindgen::rt::{alloc, vec::Vec, string::String};
  
  // Before executing any other code, use this function to run all static
  // constructors, if they have not yet been run. This is a hack required
  // to work around wasi-libc ctors calling import functions to initialize
  // the environment.
  //
  // This functionality will be removed once rust 1.69.0 is stable, at which
  // point wasi-libc will no longer have this behavior.
  //
  // See
  // https://github.com/bytecodealliance/preview2-prototyping/issues/99
  // for more details.
  #[cfg(target_arch="wasm32")]
  wit_bindgen::rt::run_ctors_once();
  
  let len0 = arg1 as usize;
  let result1 = T::something({#[cfg(not(debug_assertions))]{String::from_utf8_unchecked(Vec::from_raw_parts(arg0 as *mut _, len0, len0))}#[cfg(debug_assertions)]{String::from_utf8(Vec::from_raw_parts(arg0 as *mut _, len0, len0)).unwrap()}});
  let ptr2 = _RET_AREA.0.as_mut_ptr() as i32;
  let vec3 = (result1.into_bytes()).into_boxed_slice();
  let ptr3 = vec3.as_ptr() as i32;
  let len3 = vec3.len() as i32;
  ::core::mem::forget(vec3);
  *((ptr2 + 4) as *mut i32) = len3;
  *((ptr2 + 0) as *mut i32) = ptr3;
  ptr2
}

#[doc(hidden)]
pub unsafe fn post_return_something<T: Component>(arg0: i32,) {
  wit_bindgen::rt::dealloc(*((arg0 + 0) as *const i32), (*((arg0 + 4) as *const i32)) as usize, 1);
}

#[allow(unused_imports)]
use wit_bindgen::rt::{alloc, vec::Vec, string::String};

#[repr(align(4))]
struct _RetArea([u8; 8]);
static mut _RET_AREA: _RetArea = _RetArea([0; 8]);

/// Declares the export of the component's world for the
/// given type.
#[macro_export]
macro_rules! export_component(($t:ident) => {
  const _: () = {
    
    #[doc(hidden)]
    #[export_name = "something"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __export_something(arg0: i32,arg1: i32,) -> i32 {
      call_something::<$t>(arg0,arg1,)
    }
    
    #[doc(hidden)]
    #[export_name = "cabi_post_something"]
    #[allow(non_snake_case)]
    unsafe extern "C" fn __post_return_something(arg0: i32,) {
      post_return_something::<$t>(arg0,)
    }
    
  };
  
  #[used]
  #[doc(hidden)]
  #[cfg(target_arch = "wasm32")]
  static __FORCE_SECTION_REF: fn() = __link_section;
});

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:component"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 196] = [3, 0, 9, 99, 111, 109, 112, 111, 110, 101, 110, 116, 0, 97, 115, 109, 13, 0, 1, 0, 7, 67, 1, 65, 2, 1, 65, 2, 1, 64, 1, 1, 115, 115, 0, 115, 4, 0, 9, 115, 111, 109, 101, 116, 104, 105, 110, 103, 1, 0, 4, 1, 34, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 101, 120, 97, 109, 112, 108, 101, 58, 100, 101, 109, 111, 47, 99, 111, 109, 112, 111, 110, 101, 110, 116, 4, 0, 0, 69, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 49, 49, 46, 48, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 5, 48, 46, 55, 46, 48, 11, 34, 1, 1, 28, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 101, 120, 97, 109, 112, 108, 101, 58, 100, 101, 109, 111, 47, 119, 105, 116, 3, 0, 0];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}

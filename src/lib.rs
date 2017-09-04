#[cfg(target_os = "android")]
extern crate android_ffi;
// extern crate kinito;

use std::boxed::Box;

#[no_mangle]
#[inline(never)]
#[allow(non_snake_case)]
#[cfg(target_os = "android")]
pub extern "C" fn rust_android_main(app: *mut ()) {
    android_ffi::android_main2(app as *mut _, move |c, v| unsafe { main(c, v) });
}

#[cfg(target_os = "android")]
fn main(_: isize, _: *const *const u8) {}

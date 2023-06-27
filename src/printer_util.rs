use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use rand::Rng;

#[no_mangle]
pub extern fn rust_doing_sth(to: *const c_char) -> *mut c_char {

    let c_str = unsafe { CStr::from_ptr(to) };
    let reciever = match c_str.to_str() {
        Err(_) => "error",
        Ok(string) => string,
    };
    let num = rand::thread_rng().gen_range(1000..9999);
    let numString = num.to_string();

    CString::new(reciever.to_owned() +
        Box::leak(numString.into_boxed_str())).unwrap().into_raw()
}

#[cfg(target_os="android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;
    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jstring};

    #[no_mangle]
    pub unsafe extern fn Java_cn_com_sony_greetings_RustBridge_doSth(env: JNIEnv, _: JClass, java_pattern: JString) ->jstring {
        let info = rust_doing_sth(env.get_string(java_pattern).expect("invalid").as_ptr());
        let info_ptr = CString::from_raw(info);
        let output = env.new_string(info_ptr.to_str().unwrap()).expect("can't new string from android func");
        output.into_inner()
    }
}

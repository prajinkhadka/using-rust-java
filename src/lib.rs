use std::fs;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use simd_json::to_owned_value;

#[no_mangle]
pub extern "system" fn Java_RustJsonParser_parseJson(
    mut env: JNIEnv,
    _class: JClass,
    jpath: JString,
) -> jstring {
    let path: String = match env.get_string(&jpath) {
        Ok(p) => p.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let data = match fs::read(&path) {
        Ok(d) => d,
        Err(_) => return null_string(&env, "error"),
    };

    let mut json_data = data.clone();
    let result = match to_owned_value(&mut json_data) {
        Ok(_) => "parsed",
        Err(_) => "error",
    };

    match env.new_string(result) {
        Ok(jstr) => jstr.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

fn null_string(env: &JNIEnv, msg: &str) -> jstring {
    env.new_string(msg).unwrap_or_default().into_raw()
}

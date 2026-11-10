use jni::objects::{JClass, JString};
use jni::EnvUnowned;

#[unsafe(no_mangle)]
pub extern "C" fn host_api_init() -> i32 {
    1
}

/// Fungsi Rust JNI untuk callback keyboard
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_habicode_hostapi_HostApiBridge_nativeShowKeyboard(
    _unowned_env: EnvUnowned<'_>,
    _class: JClass,
) {
    // Logika native diteruskan atau di-handle bersih
}

/// Fungsi Rust JNI untuk clipboard
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_habicode_hostapi_HostApiBridge_nativeSetClipboard(
    mut unowned_env: EnvUnowned<'_>,
    _class: JClass,
    text: JString,
) {
    let _ = unowned_env.with_env(|env| -> Result<(), jni::errors::Error> {
        let _input_str: String = text.try_to_string(env)?;
        Ok(())
    });
}

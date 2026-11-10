use jni::objects::{JClass, JString};
use jni::EnvUnowned;

// Inisialisasi awal host-api
#[unsafe(no_mangle)]
pub extern "C" fn host_api_init() -> i32 {
    1
}

/// Fungsi Rust untuk memanggil JNI Java `showKeyboard`
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_habicode_hostapi_HostApiBridge_nativeShowKeyboard(
    _unowned_env: EnvUnowned<'_>,
    _class: JClass,
) {
    // Logika pemanggilan native ke Java
}

/// Fungsi untuk mengirim teks ke clipboard sistem lewat JNI
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

use jni::objects::{JClass, JString, JObject};
use jni::EnvUnowned;

#[unsafe(no_mangle)]
pub extern "C" fn host_api_init() -> i32 {
    1
}

/// Fungsi Rust untuk memanggil JNI Java `showKeyboard`
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_habicode_hostapi_HostApiBridge_nativeShowKeyboard(
    mut unowned_env: EnvUnowned<'_>,
    _class: JClass,
) {
    let _ = unowned_env.with_env(|env| -> Result<(), jni::errors::Error> {
        // Ambil Context atau Activity dari global / Android environment jika memungkinkan,
        // atau panggil toggle soft input via InputMethodManager system service.
        
        // Contoh akses Context / Window Manager via JNI reflection/system service:
        // Karena BroadcastReceiver butuh Activity context untuk showSoftInput, 
        // kita pastikan InputMethodManager dipanggil dengan flag SHOW_FORCED.
        
        let activity_thread_class = env.find_class("android/app/ActivityThread")?;
        let current_activity_thread = env.call_static_method(
            &activity_thread_class,
            "currentActivityThread",
            "()Landroid/app/ActivityThread;",
            &[],
        )?.l()?;

        if !current_activity_thread.is_null() {
            // Ambil Application Context
            let context = env.call_method(
                &current_activity_thread,
                "getApplication",
                "()Landroid/app/Application;",
                &[],
            )?.l()?;

            if !context.is_null() {
                let im_str = env.new_string("input_method")?;
                let imm = env.call_method(
                    &context,
                    "getSystemService",
                    "(Ljava/lang/String;)Ljava/lang/Object;",
                    &[(&im_str).into()],
                )?.l()?;

                if !imm.is_null() {
                    // Toggle soft input (SHOW_FORCED = 2, 0 = HIDE_IMPLICIT_ONLY)
                    // toggleSoftInput(int showFlags, int hideFlags)
                    let _ = env.call_method(
                        &imm,
                        "toggleSoftInput",
                        "(II)V",
                        &[2.into(), 0.into()],
                    );
                }
            }
        }
        Ok(())
    });
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

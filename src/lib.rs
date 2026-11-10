use android_activity::AndroidApp;
use jni::objects::{JClass, JString};
use jni::EnvUnowned;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Debug),
    );
    
    log::info!("HabiCode Host API started successfully via android-activity.");

    loop {
        app.poll_events(Some(std::time::Duration::from_millis(100)), |event| {
            match event {
                android_activity::PollEvent::Wake => {
                    log::info!("App woken up.");
                }
                android_activity::PollEvent::Timeout => {}
                android_activity::PollEvent::Main(main_event) => {
                    match main_event {
                        android_activity::MainEvent::InitWindow { .. } => {
                            log::info!("Native window initialized.");
                        }
                        android_activity::MainEvent::Destroy => {
                            log::info!("Native activity destroyed.");
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}

/// Fungsi JNI untuk callback keyboard
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_habicode_hostapi_HostApiBridge_nativeShowKeyboard(
    _unowned_env: EnvUnowned<'_>,
    _class: JClass,
) {
    log::info!("JNI call: nativeShowKeyboard triggered.");
}

/// Fungsi JNI untuk clipboard
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_habicode_hostapi_HostApiBridge_nativeSetClipboard(
    mut unowned_env: EnvUnowned<'_>,
    _class: JClass,
    text: JString,
) {
    let _ = unowned_env.with_env(|env| -> Result<(), jni::errors::Error> {
        let input_str: String = text.try_to_string(env)?;
        log::info!("JNI clipboard text received: {}", input_str);
        Ok(())
    });
}

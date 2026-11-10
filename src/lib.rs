use android_activity::AndroidApp;
use jni::objects::JClass;
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
                android_activity::PollEvent::Wake => {}
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

/// Fungsi JNI yang dipanggil oleh HostApiReceiver saat ada broadcast masuk
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_habicode_hostapi_HostApiReceiver_nativeShowKeyboardFromJava(
    _unowned_env: EnvUnowned<'_>,
    _class: JClass,
) {
    log::info!("SUCCESS: Broadcast received from Termux! Triggering keyboard logic...");
    
    // Di sinilah nanti kita bisa menyisipkan logika InputMethodManager atau event sistem
}

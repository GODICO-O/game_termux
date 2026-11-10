package com.habicode.hostapi;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.util.Log;

public class HostApiReceiver extends BroadcastReceiver {
    private static final String TAG = "HostApiReceiver";

    @Override
    public void onReceive(Context context, Intent intent) {
        if (intent == null) return;
        
        String action = intent.getAction();
        Log.d(TAG, "Received broadcast action: " + action);

        if ("com.habicode.hostapi.SHOW_KEYBOARD".equals(action)) {
            Log.d(TAG, "Triggering nativeShowKeyboard from Rust...");
            // Panggil fungsi native Rust yang sudah kita siapkan
            nativeShowKeyboardFromJava();
        }
    }

    // Deklarasi native method yang terhubung ke libhost_api.so
    private native static void nativeShowKeyboardFromJava();
}

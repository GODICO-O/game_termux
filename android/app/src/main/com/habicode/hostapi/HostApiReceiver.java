package com.habicode.hostapi;

import android.content.BroadcastReceiver;
import android.content.Context;
import android.content.Intent;
import android.view.inputmethod.InputMethodManager;
import android.util.Log;

public class HostApiReceiver extends BroadcastReceiver {
    private static final String TAG = "HostApiReceiver";

    @Override
    public void onReceive(Context context, Intent intent) {
        if (intent == null) {
            Log.e(TAG, "Received null intent");
            return;
        }

        String action = intent.getAction();
        Log.d(TAG, "Received broadcast action: " + action);

        if ("com.habicode.hostapi.SHOW_KEYBOARD".equals(action)) {
            try {
                // Memanggil HostApiBridge seperti ala Termux-API
                HostApiBridge.showKeyboard(context);
            } catch (Exception e) {
                Log.e(TAG, "Error handling SHOW_KEYBOARD broadcast", e);
            }
        }
    }
}

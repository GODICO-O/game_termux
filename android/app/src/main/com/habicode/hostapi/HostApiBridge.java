package com.habicode.hostapi;

import android.content.Context;
import android.view.inputmethod.InputMethodManager;
import android.util.Log;
import android.os.IBinder;

public class HostApiBridge {
    private static final String TAG = "HostApiBridge";

    public static void showKeyboard(Context context) {
        try {
            Log.d(TAG, "Attempting to force show keyboard...");
            InputMethodManager imm = (InputMethodManager) context.getSystemService(Context.INPUT_METHOD_SERVICE);
            if (imm != null) {
                // Menggunakan metode modern untuk memaksa menampilkan input method
                imm.showSoftInputFromInputMethod(null, InputMethodManager.SHOW_FORCED);
            }
        } catch (Exception e) {
            Log.e(TAG, "Error showing keyboard: " + e.getMessage(), e);
        }
    }
}

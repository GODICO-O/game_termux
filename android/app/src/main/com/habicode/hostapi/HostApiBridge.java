package com.habicode.hostapi;

import android.content.Context;
import android.view.inputmethod.InputMethodManager;
import android.util.Log;

public class HostApiBridge {
    private static final String TAG = "HostApiBridge";

    public static void showKeyboard(Context context) {
        try {
            Log.d(TAG, "Executing showKeyboard from context: " + context);
            InputMethodManager imm = (InputMethodManager) context.getSystemService(Context.INPUT_METHOD_SERVICE);
            if (imm != null) {
                // Menggunakan toggleSoftInput dengan cara aman
                imm.toggleSoftInput(InputMethodManager.SHOW_FORCED, 0);
                Log.d(TAG, "toggleSoftInput command executed successfully.");
            } else {
                Log.e(TAG, "InputMethodManager service is null!");
            }
        } catch (Exception e) {
            Log.e(TAG, "Failed in showKeyboard: " + e.getMessage(), e);
        }
    }
}

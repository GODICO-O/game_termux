package com.habicode.hostapi;

import android.content.Context;
import android.view.inputmethod.InputMethodManager;
import android.content.ClipData;
import android.content.ClipboardManager;

public class HostApiBridge {

    // Fungsi untuk memunculkan Gboard / Software Keyboard secara paksa
    public static void showKeyboard(Context context) {
        if (context != null) {
            InputMethodManager imm = (InputMethodManager) context.getSystemService(Context.INPUT_METHOD_SERVICE);
            if (imm != null) {
                // Toggle untuk memunculkan soft input
                imm.toggleSoftInput(InputMethodManager.SHOW_FORCED, 0);
            }
        }
    }

    // Fungsi untuk menyembunyikan keyboard
    public static void hideKeyboard(Context context, android.view.View view) {
        if (context != null && view != null) {
            InputMethodManager imm = (InputMethodManager) context.getSystemService(Context.INPUT_METHOD_SERVICE);
            if (imm != null) {
                imm.hideSoftInputFromWindow(view.getWindowToken(), 0);
            }
        }
    }

    // Fungsi untuk menyalin teks ke System Clipboard Android
    public static void setClipboard(Context context, String text) {
        if (context != null) {
            ClipboardManager clipboard = (ClipboardManager) context.getSystemService(Context.CLIPBOARD_SERVICE);
            ClipData clip = ClipData.newPlainText("HabiCode Clipboard", text);
            clipboard.setPrimaryClip(clip);
        }
    }
}

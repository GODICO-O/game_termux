package com.habicode.hostapi;

import android.app.Activity;
import android.os.Bundle;
import android.util.Log;

public class MainActivity extends Activity {
    private static final String TAG = "HostApiMain";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Log.d(TAG, "HostApi MainActivity started successfully.");
        // Langsung tutup otomatis supaya tidak mengganggu layar, tapi status app sudah aktif
        finish();
    }
}

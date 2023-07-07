
// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;
import fr.stardustenterprises.yanl.NativeLoader;

public class TeraRusty {
    private static final NativeLoader loader = new NativeLoader.Builder()
        .build();

    static {
        loader.loadLibrary("tera-rusty", false);
    }

    // window init code
    public static native void initializeWindowX11(long displayptr, long windowptr);
}

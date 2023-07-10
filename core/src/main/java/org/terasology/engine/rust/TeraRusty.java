
// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

public class TeraRusty {
    static {
        NativeSupport.load("core_rust");
    }

    private TeraRusty() {

    }

    // window init code
    public static native void initializeWindowX11(long displayptr, long windowptr);
    public static native void initializeWindowWin32(long displayptr, long windowptr);

    public static native void windowSizeChanged(int width, int height);

    public static native void dispatch();

}

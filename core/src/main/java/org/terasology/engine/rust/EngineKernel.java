// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

public final  class EngineKernel implements Disposable {
    private static final class JNI {
        private static native long create();
        private static native void drop(long rustPtr);
        private static native void initSurfaceWin32(long kernel, long displayHandle, long windowHandle);
        private static native void initSurfaceX11(long kernel, long displayHandle, long windowHandle);
        private static native void resizeSurface(long kernel, int width, int height);
    }

    long rustKernelPtr = 0;

    static {
        NativeSupport.load("core_rust");
    }
    @Override
    public void dispose() {
        JNI.drop(this.rustKernelPtr);
        this.rustKernelPtr = 0;
    }

    public EngineKernel() {
        this.rustKernelPtr = JNI.create();
    }

    public void initializeWin32Surface(long display, long window) {
        JNI.initSurfaceWin32(rustKernelPtr, display, window);
    }
    public void initializeWinX11Surface(long display, long window) {
        JNI.initSurfaceX11(rustKernelPtr, display, window);
    }

    public void resizeSurface(int width, int height) {
        JNI.resizeSurface(rustKernelPtr, width, height);
    }



}


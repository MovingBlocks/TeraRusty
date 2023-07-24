// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

import java.lang.ref.Cleaner;

public final class EngineKernel implements Disposable {
    static final Cleaner CLEANER = Cleaner.create();

    final long rustKernelPtr;
    private final Cleaner.Cleanable cleanable;
    public final UIRenderer ui;
    public final ResourceManager resource;


    public static final class EngineKernelBuild {
        private long displayHandle;
        private long windowHandle;
        private int windowType;

        public enum WindowType {
            Win32,
            X11
        }

        public EngineKernelBuild configureX11Window(long windowHandle, long displayHandle) {
            this.windowType = WindowType.X11.ordinal();
            this.displayHandle = displayHandle;
            this.windowHandle = windowHandle;
            return this;
        }

        public EngineKernelBuild configureWin32Window(long windowHandle, long displayHandle) {
            this.windowType = WindowType.Win32.ordinal();
            this.displayHandle = displayHandle;
            this.windowHandle = windowHandle;
            return this;
        }
    }

    public EngineKernel(EngineKernelBuild builder) {
        long kernelPtr = JNI.create(builder);
        rustKernelPtr = kernelPtr;
        this.ui = new UIRenderer(this);
        this.resource = new ResourceManager(this);
        this.cleanable = CLEANER.register(this, () -> {
            JNI.drop(kernelPtr);
        });
    }

    static {
        NativeSupport.load("core_rust");
    }


    public void resizeSurface(int width, int height) {
        JNI.resizeSurface(rustKernelPtr, width, height);
    }
    public void cmdPrepare() {
        JNI.cmdPrepare(rustKernelPtr);
    }
    public void cmdDispatch() {
        JNI.cmdDispatch(rustKernelPtr);
    }

    @Override
    public void dispose() {
        this.cleanable.clean();
    }

    private static final class JNI {
        private static native long create(EngineKernelBuild builder);

        private static native void drop(long rustPtr);

        private static native void resizeSurface(long kernel, int width, int height);
        private static native void cmdPrepare(long kernel);
        private static native void cmdDispatch(long kernel);


    }
}


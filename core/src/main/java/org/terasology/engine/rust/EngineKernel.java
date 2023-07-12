// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

import java.lang.ref.Cleaner;

public final class EngineKernel implements Disposable {
    static final Cleaner CLEANER = Cleaner.create();
    private static  EngineKernel kernel = null;
    // TODO: we might want to rework this from a singleton
    public static EngineKernel instance() {
        if (kernel == null) {
            kernel = new EngineKernel();
        }
        return kernel;
    }

    public static void initialize() {
        if (kernel != null) {
            kernel.dispose();
        }
        kernel = new EngineKernel();
    }

    public static void disposeKernel() {
        if(kernel != null) {
            kernel.dispose();
            kernel = null;
        }
    }

    private final Cleaner.Cleanable cleanable;
    final long rustKernelPtr;

    static {
        NativeSupport.load("core_rust");
    }

    private EngineKernel() {
        long kernelPtr = JNI.create();
        rustKernelPtr = kernelPtr;
        this.cleanable = CLEANER.register(this, () -> {
            if (kernelPtr != 0) {
                JNI.drop(kernelPtr);
            }
        });
    }

    public <O,T extends EngineSubsystem<O>> T addSubsystem(Class<T> classZ, O options) throws Exception {
        try {
            return classZ.getDeclaredConstructor(EngineKernel.class, options.getClass())
                    .newInstance(this, options);
        } catch (Exception e) {
            throw new Exception("Failed to initialize subsystem");
        }
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

    // resource manage
    public GeometryHandle createGeometry() {
        return new GeometryHandle();
    }

    @Override
    public void dispose() {
        this.cleanable.clean();
    }

    private static final class JNI {
        private static native long create();

        private static native void drop(long rustPtr);

        private static native void initSurfaceWin32(long kernel, long displayHandle, long windowHandle);

        private static native void initSurfaceX11(long kernel, long displayHandle, long windowHandle);

        private static native void resizeSurface(long kernel, int width, int height);
    }
}


// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

import java.lang.ref.Cleaner;
import java.util.Optional;

import org.terasology.joml.geom.Rectanglef;

public final class EngineKernel implements Disposable {
    static final Cleaner CLEANER = Cleaner.create();
    private static EngineKernel kernel = null;

    final long rustKernelPtr;
    private final Cleaner.Cleanable cleanable;

    private EngineKernel() {
        long kernelPtr = JNI.create();
        rustKernelPtr = kernelPtr;
        this.cleanable = CLEANER.register(this, () -> {
            JNI.drop(kernelPtr);
        });
    }

    // TODO: we might want to rework this from a singleton
    public static EngineKernel instance() {
        return kernel;
    }

    public static void initialize() {
        disposeKernel();
        kernel = new EngineKernel();
    }

    public static void disposeKernel() {
        if (kernel != null) {
            kernel.dispose();
            kernel = null;
        }
    }

    static {
        NativeSupport.load("core_rust");
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

    // User Interface
    public void cmdUISetCrop(Optional<Rectanglef> rect) {
        if (rect.isPresent()) {
            Rectanglef r = rect.get();
            JNI.cmdUISetCrop(rustKernelPtr, r.minX(), r.minY(), r.maxX(), r.maxY());
        } else {
            JNI.cmdUIClearCrop(rustKernelPtr);
        }
    }

    public void cmdUIDrawTexture(TeraTexture tex,Rectanglef uv, Rectanglef pos) {
        JNI.cmdUIDrawTexture(
                rustKernelPtr,
                tex.rustTexturePtr,
                uv.minX(), uv.minY(), uv.maxX(), uv.maxY(),
                pos.minX(), pos.minY(), pos.maxX(), pos.maxY()
                );
    }

    // Dispatch
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
        private static native long create();
        private static native void drop(long rustPtr);

        private static native void initSurfaceWin32(long kernel, long displayHandle, long windowHandle);

        private static native void initSurfaceX11(long kernel, long displayHandle, long windowHandle);

        private static native void resizeSurface(long kernel, int width, int height);

        private static native void cmdPrepare(long kernel);
        private static native void cmdDispatch(long kernel);

        // User Interface
        public static native void cmdUISetCrop(long kernel, float minX, float minY, float maxX, float maxY);
        public static native void cmdUIClearCrop(long kernel);
        public static native void cmdUIDrawTexture(long kernel,
           long texturePtr,
           float uvMinX, float uvMinY, float uvMaxX, float uvMaxY,
            float posMinX, float posMinY, float posMaxX, float posMaxY);
    }
}


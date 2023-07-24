// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

import org.terasology.joml.geom.Rectanglef;

import java.util.Optional;

public class UIRenderer {
    private final EngineKernel kernel;
    public UIRenderer(EngineKernel kernel) {
        this.kernel = kernel;
    }

    // User Interface
    public void cmdUISetCrop(Optional<Rectanglef> rect) {
        if (rect.isPresent()) {
            Rectanglef r = rect.get();
            UIRenderer.JNI.cmdUISetCrop(this.kernel.rustKernelPtr, r.minX(), r.minY(), r.maxX(), r.maxY());
        } else {
            UIRenderer.JNI.cmdUIClearCrop(this.kernel.rustKernelPtr);
        }
    }

    public void cmdUIDrawTexture(TeraTexture tex, Rectanglef uv, Rectanglef pos, int tintColor) {
        UIRenderer.JNI.cmdUIDrawTexture(
                this.kernel.rustKernelPtr,
                tex.rustTexturePtr,
                uv.minX(), uv.minY(), uv.maxX(), uv.maxY(),
                pos.minX(), pos.minY(), pos.maxX(), pos.maxY(),
                tintColor
        );
    }

    public void cmdUIDrawTexture(TeraTexture tex, Rectanglef uv, Rectanglef pos) {
        UIRenderer.JNI.cmdUIDrawTexture(
                this.kernel.rustKernelPtr,
                tex.rustTexturePtr,
                uv.minX(), uv.minY(), uv.maxX(), uv.maxY(),
                pos.minX(), pos.minY(), pos.maxX(), pos.maxY(),
                0xffffffff
        );
    }

    private static final class JNI {
        // User Interface
        public static native void cmdUISetCrop(long kernel, float minX, float minY, float maxX, float maxY);
        public static native void cmdUIClearCrop(long kernel);
        public static native void cmdUIDrawTexture(long kernel,
                                                   long texturePtr,
                                                   float uvMinX, float uvMinY, float uvMaxX, float uvMaxY,
                                                   float posMinX, float posMinY, float posMaxX, float posMaxY,
                                                   int tintColor);
    }
}

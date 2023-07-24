// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

import org.joml.Vector2f;
import org.joml.Vector2fc;

import java.lang.ref.Cleaner;

import static org.terasology.engine.rust.EngineKernel.CLEANER;

public class TeraTexture implements Disposable {
    final long rustTexturePtr;
    private final Cleaner.Cleanable cleanable;
    private final EngineKernel kernel;
    private Vector2f size = new Vector2f();

    TeraTexture(EngineKernel kernel, long texturePtr) {
        this.kernel = kernel;
        rustTexturePtr = texturePtr;
        this.cleanable = CLEANER.register(this, () -> {
            TeraTexture.JNI.drop(texturePtr);
        });
    }

    public enum TextureDimension {
        DIM_1D,
        DIM_2D,
        DIM_3D
    }

    public enum ImageFormat {
        UNKNOWN,
        R8_UNORM,
        R8_SNORM,
        R8_UINT,
        R8_SINT,
        R8G8_UNORM,
        R8G8_SNORM,
        R8G8_UINT,
        R8G8_SINT,
        R16_UNORM,
        R16_SNORM,
        R16_UINT,
        R16_SINT,
        R8G8B8A8_UNORM,
        R8G8B8A8_SNORM,
        R8G8B8A8_UINT,
        R8G8B8A8_SINT,
    }

    // Texture Resource
    // TODO: this is what we want to use instead TextureDesc is a generate structure that describes a texture instead of all these methods for creating texture resource
    public static final class TextureDesc {
        int width;
        int height;
        int layers;
        int dim;
        int format;

        public TextureDesc setWidth(int width) {
            this.width = width;
            return this;
        }

        public TextureDesc setHeight(int height) {
            this.height = height;
            return this;
        }

        public TextureDesc setLayers(int layers) {
            this.layers = layers;
            return this;
        }

        public TextureDesc setFormat(ImageFormat format) {
            this.format = format.ordinal();
            return this;
        }

        public TextureDesc setDim(TextureDimension dim) {
            this.dim = dim.ordinal();
            return this;
        }


    }


    public void writeTextureBuffer(java.nio.ByteBuffer buffer) {
        JNI.writeTextureBuffer(kernel.rustKernelPtr, this.rustTexturePtr, buffer);
    }

    public Vector2fc getSize() {
        JNI.getSize(this.rustTexturePtr, this.size);
        return this.size;
    }

    @Override
    public void dispose() {
        this.cleanable.clean();
    }


    private static final class JNI {
        private static native void drop(long rustPtr);

        public static native void getSize(long textureResourcePtr, Vector2f vec);
        public static native void writeTextureBuffer(long kernelPtr, long textureResourcePtr, java.nio.ByteBuffer buffer);

    }
}

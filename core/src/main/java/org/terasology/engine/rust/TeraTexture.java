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
    private Vector2f size = new Vector2f();

    TeraTexture(long texturePtr) {
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
        public int width;
        public int height;
        public int layers;

        public TextureDimension dim;
        public ImageFormat format;
    }

    public static TeraTexture create(TextureDesc desc) {
        EngineKernel instance = EngineKernel.instance();
        return new TeraTexture(JNI.createTextureResource(instance.rustKernelPtr, new JNI.TextureDescJNI(desc)));
    }
    public static TeraTexture createFromBuffer(TextureDesc desc, java.nio.ByteBuffer buffer) {
        EngineKernel instance = EngineKernel.instance();
        return new TeraTexture(JNI.createTextureResourceFromBuffer(instance.rustKernelPtr, new JNI.TextureDescJNI(desc), buffer));
    }

    public void writeTextureBuffer(java.nio.ByteBuffer buffer) {
        EngineKernel instance = EngineKernel.instance();
        JNI.writeTextureBuffer(instance.rustKernelPtr, this.rustTexturePtr, buffer);
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
        // use a byte buffer ...
        static final class TextureDescJNI {
            public TextureDescJNI(TextureDesc desc) {
                this.format = desc.format.ordinal();
                this.dim = desc.dim.ordinal();
                this.width = desc.width;
                this.height = desc.height;
                this.layers = desc.layers;
            }
            public int width;
            public int height;
            public int layers;
            public int dim;
            public int format;
        }

        private static native void drop(long rustPtr);

        public static native void getSize(long textureResourcePtr, Vector2f vec);
        public static native void writeTextureBuffer(long kernelPtr, long textureResourcePtr, java.nio.ByteBuffer buffer);

        // Resource
        public static native long createTextureResourceFromBuffer(
                long kernelPtr, TextureDescJNI desc, java.nio.ByteBuffer buffer);
        public static native long createTextureResource(long kernelPtr, TextureDescJNI desc);

    }
}

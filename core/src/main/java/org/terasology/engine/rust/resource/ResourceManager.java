// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust.resource;

import org.terasology.engine.rust.EngineKernel;

public class ResourceManager {
    private final EngineKernel kernel;
    public ResourceManager(EngineKernel kernel) {
        this.kernel = kernel;
    }

    public TeraTexture createTexture(TeraTexture.TextureDesc desc) {
        return new TeraTexture(this.kernel, ResourceManager.JNI.createTextureResource(this.kernel.getHandle(), desc));
    }
    public TeraTexture createTexture(TeraTexture.TextureDesc desc, java.nio.ByteBuffer buffer) {
        return new TeraTexture(this.kernel, ResourceManager.JNI.createTextureResourceFromBuffer(this.kernel.getHandle(), desc, buffer));
    }

    public ChunkGeometry createChunkGeometry() {
        return new ChunkGeometry(this.kernel, ResourceManager.JNI.createChunkResource(this.kernel.getHandle()));
    }

    private static class JNI {
        public static native long createTextureResourceFromBuffer(long kernelPtr, TeraTexture.TextureDesc desc, java.nio.ByteBuffer buffer);
        public static native long createTextureResource(long kernelPtr,  TeraTexture.TextureDesc desc);
        public static native long createChunkResource(long kernelPtr);
    }
}

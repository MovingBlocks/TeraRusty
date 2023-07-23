// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

public class ResourceManager {
    private final EngineKernel kernel;
    public ResourceManager(EngineKernel kernel) {
        this.kernel = kernel;
    }

    public TeraTexture createTexture(TeraTexture.TextureDesc desc) {
        return new TeraTexture(this.kernel, ResourceManager.JNI.createTextureResource(this.kernel.rustKernelPtr, desc));
    }
    public TeraTexture createTexture(TeraTexture.TextureDesc desc, java.nio.ByteBuffer buffer) {
        return new TeraTexture(this.kernel, ResourceManager.JNI.createTextureResourceFromBuffer(this.kernel.rustKernelPtr, desc, buffer));
    }

    private static class JNI {
        public static native long createTextureResourceFromBuffer(long kernelPtr, TeraTexture.TextureDesc desc, java.nio.ByteBuffer buffer);
        public static native long createTextureResource(long kernelPtr,  TeraTexture.TextureDesc desc);

    }
}

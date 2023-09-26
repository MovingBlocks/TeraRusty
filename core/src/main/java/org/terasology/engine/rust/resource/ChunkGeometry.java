// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust.resource;

import org.terasology.engine.rust.EngineKernel;
import org.terasology.engine.rust.RawHandle;

import java.lang.ref.Cleaner;

public class ChunkGeometry implements RawHandle<ChunkGeometry> {
    final long rustPtr;
    final EngineKernel kernel;
    private final Cleaner.Cleanable cleanable;

    public enum MeshRenderType {
        Opaque,
        Translucent,
        Billboard,
        WaterAndIce
    }

    public ChunkGeometry(EngineKernel kernel, long rustPtr) {
        this.rustPtr = rustPtr;
        this.kernel = kernel;
        this.cleanable = EngineKernel.CLEANER.register(this, () -> {
            ChunkGeometry.JNI.drop(rustPtr);
        });
    }

    public void setMeshResource(
            java.nio.ByteBuffer index,
            java.nio.ByteBuffer position,
            java.nio.ByteBuffer normal,
            java.nio.ByteBuffer uv,
            java.nio.ByteBuffer color,
            java.nio.ByteBuffer attributes
    ) {
        JNI.setMeshResource(this.kernel.getHandle(), rustPtr, index, position, normal, uv, color, attributes);
    }

    @Override
    public long getHandle() {
        return rustPtr;
    }

    private static final class JNI {
        static native void drop(long rustPtr);

        static native void setMeshResource(long kernelPtr, long chunkPtr,
                                           java.nio.ByteBuffer index,
                                           java.nio.ByteBuffer position,
                                           java.nio.ByteBuffer normal,
                                           java.nio.ByteBuffer uv,
                                           java.nio.ByteBuffer color,
                                           java.nio.ByteBuffer attributes
        );

        static native void clearMeshResource(long kernelPtr, long chunkPtr);

    }
}

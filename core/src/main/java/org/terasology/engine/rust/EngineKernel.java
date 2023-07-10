// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

public class EngineKernel implements AutoCloseable {
    protected long rustPtr = 0;
    public enum Backend {
        DIRECTX11((byte)0b000001);

        private final byte flag;
        Backend(byte flag) {
            this.flag = flag;
        }
    }

    public Core(long display, long window) {
        this.rustPtr = createInst();
    }

    private static native long createInst();
    private static native void disposeInst(long rustPtr);

    @Override
    public void close() throws Exception {
        disposeInst(this.rustPtr);
        this.rustPtr = 0;
    }
}


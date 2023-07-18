// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

public class NativeWindow implements AutoCloseable {
    protected long rustPtr = 0;
    public NativeWindow() {
        rustPtr = create();
    }



    private static native long create();
    private static native void dispose(long rustPtr);

    @Override
    public void close() throws Exception {
        dispose(rustPtr);
    }
}

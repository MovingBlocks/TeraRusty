// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

public class GeometryHandle implements Disposable {
    GeometryHandle() {

    }

    @Override
    public void dispose() {
    }

    private static final class JNI {
        private static native long create();
        private static native void drop(long rustPtr);
    }

}


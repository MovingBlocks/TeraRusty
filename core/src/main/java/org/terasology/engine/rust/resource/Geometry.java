// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust.resource;

import org.terasology.engine.rust.Disposable;

public class Geometry implements Disposable {
    Geometry() {

    }

    @Override
    public void dispose() {
    }

    private static final class JNI {
        private static native void drop(long rustPtr);
    }

}


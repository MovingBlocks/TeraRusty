// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

public abstract class EngineSubsystem<T> {
    protected final EngineKernel kernel;
    EngineSubsystem(EngineKernel kernel, T ignoredOptions) {
        this.kernel = kernel;
    }
}

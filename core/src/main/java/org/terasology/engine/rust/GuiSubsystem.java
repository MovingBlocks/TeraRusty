// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

package org.terasology.engine.rust;

import java.lang.ref.Cleaner;

public class GuiSubsystem extends EngineSubsystem<GuiSubsystem.Options> {

    private final Cleaner.Cleanable cleanable;
    final long rustGuiSubsystemPtr;

    GuiSubsystem(EngineKernel kernel, Options options) {
        super(kernel, options);
        long guiSubsystemPtr = GuiSubsystem.JNI.create(kernel.rustKernelPtr);
        rustGuiSubsystemPtr = guiSubsystemPtr;

        this.cleanable = EngineKernel.CLEANER.register(this, () -> {
            if (rustGuiSubsystemPtr != 0) {
                GuiSubsystem.JNI.drop(rustGuiSubsystemPtr);
            }
        });
    }

    public static final class Options  {
    }

    private static final class JNI {
        private static native long create(long kernelPtr);
        private static native void drop(long rustPtr);

    }
}

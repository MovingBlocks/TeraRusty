// Copyright 2023 The Terasology Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::Error;
use jni::JNIEnv;

pub fn try_throw<T>(env: &mut JNIEnv, block: impl FnOnce(&mut JNIEnv) -> anyhow::Result<T>) -> T
    where T: Default {
    block(env)
        .unwrap_or_else(|err: Error| {
            env.throw(format!("{}\nNative Error backtrace: {}",err,err.backtrace()))
                .expect("Cannot throw exception to Java, Sorry T_T");
            T::default() // Don't matter, because we throw an exception
        })
}


use jni::{sys::{JNIEnv, jlong}, objects::JClass};
use std::sync::Arc;

use crate::java_util::{JInstance, JArcInstance};

pub struct EngineKernel {
    instance: wgpu::Instance
}

impl EngineKernel {

    
    pub fn renderer(&self) -> &wgpu::Instance {
       &self.instance 
    }
}

impl JArcInstance for EngineKernel {
    type Item = EngineKernel;
    type JavaType = Arc<Self::Item>;
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_Renderer_createInst(_jni: JNIEnv, _class: JClass) -> jlong  {
    EngineKernel::java_create(EngineKernel {
        instance: wgpu::Instance::default()
    })
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_Renderer_disposeInst(_jni: JNIEnv, _class: JClass, ptr: jlong) {
    EngineKernel::java_dispose(ptr);
}



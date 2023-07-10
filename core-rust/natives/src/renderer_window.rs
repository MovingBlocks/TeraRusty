use jni::{sys::{JNIEnv, jlong}, objects::JClass};
use raw_window_handle::{XlibWindowHandle, XlibDisplayHandle, HasRawWindowHandle, RawWindowHandle, HasRawDisplayHandle, RawDisplayHandle};
use crate::engine_kernel::EngineKernel;
use crate::c_void;

use crate::java_util::{JInstance, JArcInstance};
use std::sync::Arc;

struct WindowSurface {
    kernel: Arc<EngineKernel>,
    wgpu_surface: wgpu::Surface,
    wgpu_adapter: wgpu::Adapter,
    wgpu_device: wgpu::Device,
    wgpu_graphics_queue: wgpu::Queue,
}

impl JArcInstance for WindowSurface {
    type Item = WindowSurface ;
    type JavaType = Arc<Self::Item>;
}

//impl JInstance for WindowSurface {
//    type JavaPtrType = Arc<WindowSurface>;
//    fn java_from(ptr: jlong) -> Arc<WindowSurface> {
//        unsafe { Arc::from_raw(ptr as *const WindowSurface) }
//    }
//
//    fn java_to(from: Arc<WindowSurface>) -> jlong {
//       Arc::into_raw(from) as jlong 
//    }
//}


#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_NativeWindow_createInstX11(_jni: JNIEnv, _class: JClass,
    kernel_ptr: jlong,
    display_ptr: jlong,
    window_ptr: jlong) -> jlong {
    let mut kernel = EngineKernel::from_jlong(kernel_ptr); 

    struct X11WindowSurface {
        window: XlibWindowHandle,
        display: XlibDisplayHandle
    }
    let mut window = X11WindowSurface {
        window: XlibWindowHandle::empty(),
        display: XlibDisplayHandle::empty()
    };
    

    unsafe impl HasRawWindowHandle for X11WindowSurface  {
        fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
            return RawWindowHandle::Xlib(self.window);
        }
    }

    unsafe impl HasRawDisplayHandle for X11WindowSurface {
        fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
            return RawDisplayHandle::Xlib(self.display);
        }
    }

    window.window.window = window_ptr as u64;
    window.display.display = display_ptr as *mut c_void;
  

    return 0;
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_NativeWindow_disposeInst(_jni: JNIEnv, _class: JClass, ptr: jlong)  {
}



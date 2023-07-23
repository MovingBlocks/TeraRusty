use jni::{JNIEnv, objects::{JClass, JObject}, sys::{jint, jlong}};
use raw_window_handle::{WindowsDisplayHandle, Win32WindowHandle, XlibWindowHandle, XlibDisplayHandle};
use core::ffi::{c_void, c_ulong};
use std::sync::Arc;
use crate::ui::JavaHandle;
use crate::engine_kernel::{EngineKernel, EngineKernelDesc, EngineEvent, ResizePayload};
use crate::window_surface::{WindowSurfaceDesc, WindowDesc, Win32WindowDesc, X11WindowDesc};

#[repr(u32)]
enum JavaWindowType {
    Win32,
    X11
}


#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_create<'local>(mut env: JNIEnv<'local>, _class: JClass, desc: JObject<'local>) -> jlong  {

    let windowType = env.get_field(&desc, "windowType", "I").unwrap().i().unwrap() ;
    let display_ptr = env.get_field(&desc, "displayHandle", "J").unwrap().j().unwrap() ;
    let window_ptr = env.get_field(&desc, "windowHandle", "J").unwrap().j().unwrap() ;
    let java_window_type : JavaWindowType = unsafe { std::mem::transmute::<jint, JavaWindowType>(windowType) }.into();
    
    let window_desc = match java_window_type {
        JavaWindowType::X11 => {
            let mut win = X11WindowDesc {
               window: XlibWindowHandle::empty(),
               display: XlibDisplayHandle::empty() 
            };
            win.window.window = window_ptr as c_ulong;
            win.display.display = display_ptr as *mut c_void;
            WindowDesc::X11(win)
        },
        JavaWindowType::Win32 => {
            let mut win = Win32WindowDesc {
               window: Win32WindowHandle::empty(),
               display: WindowsDisplayHandle::empty() 
            };
            win.window.hwnd = window_ptr as *mut c_void;
            WindowDesc::Win32(win)
        }
    };

     let mut window_surface_desc = WindowSurfaceDesc {
        window: window_desc 
    };

    let instance = wgpu::Instance::default();
    return EngineKernel::to_handle(Arc::new(EngineKernel::new(instance, &EngineKernelDesc {
        surface: window_surface_desc
    })));
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_drop(_jni: JNIEnv, _class: JClass, ptr: jlong) {   
     EngineKernel::drop_handle(ptr);
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_resizeSurface(_jni: JNIEnv, _class: JClass,
    kernel_ptr: jlong, width: jint, height: jint) {
    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
  
    kernel.dispatch_event(&EngineEvent::Resize(ResizePayload {
        width: width as u32,
        height: height as u32
    }));
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdPrepare(_jni: JNIEnv, _class: JClass, kernel_ptr: jlong) {
    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
    kernel.cmd_prepare();
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdDispatch(_jni: JNIEnv, _class: JClass, kernel_ptr: jlong) {
    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
    kernel.cmd_prepare();
}

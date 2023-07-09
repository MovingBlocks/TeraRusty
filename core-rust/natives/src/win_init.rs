use jni::{sys::{ JNIEnv, jlong}, objects::JClass};
use raw_window_handle::{XlibDisplayHandle, XlibWindowHandle, HasRawWindowHandle, RawWindowHandle,  HasRawDisplayHandle, RawDisplayHandle, Win32WindowHandle, WindowsDisplayHandle};
use std::ffi::c_void;

use crate::context::CONTEXT;

struct XWindow {
    window: XlibWindowHandle,
    display: XlibDisplayHandle
}

unsafe impl HasRawWindowHandle for XWindow {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        return RawWindowHandle::Xlib(self.window);
    }
}

unsafe impl HasRawDisplayHandle for XWindow {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        return RawDisplayHandle::Xlib(self.display);
    }
}



#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_TeraRusty_initializeWindowX11(_jni: JNIEnv, _class: JClass, display_ptr: jlong, window_ptr: jlong)  {
    let mut win = XWindow {
        window: XlibWindowHandle::empty(),
        display: XlibDisplayHandle::empty()
    };
    win.window.window = window_ptr as u64;
    win.display.display = display_ptr as *mut c_void;
  
    let instance = wgpu::Instance::default();
    let surface_result = unsafe {instance.create_surface(&win) };
    let surface = match surface_result {
        Ok(surface) => surface,
        Err(err) => panic!("problem creating surface: {:?}", err),    
    };
    let mut context = CONTEXT.write().unwrap(); 
    context.wgpu_instance = Some(instance); 
    context.wgpu_surface = Some(surface);
}

struct Win32Window {
    window: Win32WindowHandle,
    display: WindowsDisplayHandle
}


unsafe impl HasRawWindowHandle for Win32Window {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        return RawWindowHandle::Win32(self.window);
    }
}

unsafe impl HasRawDisplayHandle for Win32Window {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        return RawDisplayHandle::Windows(self.display);
    }
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_TeraRusty_initializeWindowWin32(_jni: JNIEnv, _class: JClass, window_ptr: jlong)  {
    let mut win = Win32Window {
        window: Win32WindowHandle::empty(),
        display: WindowsDisplayHandle::empty()
    };
    win.window.hwnd = window_ptr as *mut c_void;
  
    let instance = wgpu::Instance::default();
    let surface_result = unsafe {instance.create_surface(&win) };
    let surface = match surface_result {
        Ok(surface) => surface,
        Err(err) => panic!("problem creating surface: {:?}", err),    
    };
    let mut context = CONTEXT.write().unwrap(); 
    context.wgpu_instance = Some(instance); 
    context.wgpu_surface = Some(surface);
}

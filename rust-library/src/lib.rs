#[macro_use]
extern crate lazy_static;

mod win_init;
mod context;
//struct XWindow {
//    window: XlibWindowHandle,
//    display: XlibDisplayHandle
//}
//
//unsafe impl HasRawWindowHandle for XWindow {
//    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
//        return RawWindowHandle::Xlib(self.window);
//    }
//}
//
//unsafe impl HasRawDisplayHandle for XWindow {
//    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
//        return RawDisplayHandle::Xlib(self.display);
//    }
//}
//
//
//#[no_mangle]
//pub extern "system" fn Java_org_terasology_engine_native_InitializeWindowX11(_jni: JNIEnv, _class: JClass, display_ptr: jlong, window_ptr: jlong) {
//    let mut win = XWindow {
//        window: XlibWindowHandle::empty(),
//        display: XlibDisplayHandle::empty()
//    };
//    win.window.window = window_ptr as u64;
//    win.display.display = display_ptr as *mut c_void;
//    
//    let instance = wgpu::Instance::default();
//    let surface = unsafe { instance.create_surface(&win) };
//}



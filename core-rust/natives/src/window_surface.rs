use futures::executor::block_on;
use jni::{sys::{jlong, JNIEnv, jint}, objects::JClass};
use raw_window_handle::{XlibWindowHandle, XlibDisplayHandle, HasRawWindowHandle, RawWindowHandle, HasRawDisplayHandle, RawDisplayHandle, Win32WindowHandle, WindowsDisplayHandle};
use crate::{ engine_kernel::EngineKernel, java_util::JavaHandle};
use core::ffi::c_void;

pub struct WindowSurface {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
impl WindowSurface {
    async fn new(instance: &wgpu::Instance, surface: wgpu::Surface ) -> WindowSurface {
        let adapter = instance 
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            }).await
            .expect("Failed to find an appropriate adapter");

        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                },
                None,
            )
            .await
            .expect("Failed to create device");
        return WindowSurface {
            surface, 
            adapter, 
            device,
            queue,
        };
    }

}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceWin32(_jni: JNIEnv, _class: JClass,
    kernel_ptr: jlong,
    _: jlong,
    window_ptr: jlong) {

    assert_ne!(window_ptr, 0);
    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };

    struct Win32WindowSurface {
        window: Win32WindowHandle,
        display: WindowsDisplayHandle
    }

    let mut window = Win32WindowSurface {
        window: Win32WindowHandle::empty(),
        display: WindowsDisplayHandle::empty()
    };

    unsafe impl HasRawWindowHandle for Win32WindowSurface {
        fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
            return RawWindowHandle::Win32(self.window);
        }
    }

    unsafe impl HasRawDisplayHandle for Win32WindowSurface  {
        fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
            return RawDisplayHandle::Windows(self.display);
        }
    }
    window.window.hwnd = window_ptr as *mut c_void;
    
    let mut write_kernel = kernel.write().unwrap();
    let surface_result = unsafe {write_kernel.instance().create_surface(&window) };
    let surface = match surface_result {
        Ok(surface) => surface,
        Err(err) => panic!("problem creating surface: {:?}", err),    
    };

    let surface = WindowSurface::new(write_kernel.instance(), surface);
    write_kernel.surface = Some(block_on(surface));
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceX11(_jni: JNIEnv, _class: JClass,
    kernel_ptr: jlong,
    display_ptr: jlong,
    window_ptr: jlong) {

    assert_ne!(display_ptr, 0);
    assert_ne!(window_ptr, 0);
    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };

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
    
    let mut write_kernel = kernel.write().unwrap();
    let surface_result = unsafe {write_kernel.instance().create_surface(&window) };
    let surface = match surface_result {
        Ok(surface) => surface,
        Err(err) => panic!("problem creating surface: {:?}", err),    
    };
    let surface = WindowSurface::new(write_kernel.instance(), surface);
    write_kernel.surface = Some(block_on(surface));
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_resizeSurface(_jni: JNIEnv, _class: JClass,
    kernel_ptr: jlong, width: jint, height: jint) {
    assert!(width > 0);
    assert!(height > 0);

    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
    let write_kernel = kernel.write().unwrap();
    let Some(surface) = write_kernel.surface.as_ref() else {panic!("surface not initialized");};

    let swapchain_capabilities = surface.surface.get_capabilities(&surface.adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: width as u32,
        height: height as u32,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };
    surface.surface.configure(&surface.device, &config);
}

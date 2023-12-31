use core::ffi::{c_void, c_ulong};

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowsDisplayHandle, XlibDisplayHandle, XlibWindowHandle};

pub struct WindowSurface {
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    surface_configuration: wgpu::SurfaceConfiguration
}

pub struct Win32WindowDesc {
   pub window: Win32WindowHandle,
   pub display: WindowsDisplayHandle
}

unsafe impl HasRawWindowHandle for Win32WindowDesc {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        return RawWindowHandle::Win32(self.window);
    }
}

unsafe impl HasRawDisplayHandle for Win32WindowDesc {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        return RawDisplayHandle::Windows(self.display);
    }
}

pub struct X11WindowDesc {
   pub window: XlibWindowHandle,
   pub display: XlibDisplayHandle
}

unsafe impl HasRawWindowHandle for X11WindowDesc {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        return RawWindowHandle::Xlib(self.window);
    }
}

unsafe impl HasRawDisplayHandle for X11WindowDesc {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        return RawDisplayHandle::Xlib(self.display);
    }
}

pub enum WindowDesc {
    Win32(Win32WindowDesc),
    X11(X11WindowDesc),
    None
}

pub struct WindowSurfaceDesc {
   pub window: WindowDesc,
}

fn default_surface_configuration(surface: &wgpu::Surface, adapter: &wgpu::Adapter) -> wgpu::SurfaceConfiguration {
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    return wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: 0 as u32,
        height: 0 as u32,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    }
}

impl WindowSurface {
    pub fn surface_info(&self) -> &wgpu::SurfaceConfiguration {
        return &self.surface_configuration;
    }

    pub fn is_surface_read(&self) -> bool{
       return self.surface_configuration.width > 0 && 
        self.surface_configuration.height > 0;
    }

    pub async fn create(instance: &wgpu::Instance, desc: &WindowSurfaceDesc) -> WindowSurface {
        let surface = match &desc.window {
            WindowDesc::Win32(window_desc) => {
                let surface_result = unsafe {instance.create_surface(&window_desc) };
                match surface_result {
                    Ok(surface) => surface,
                    Err(err) => panic!("problem creating surface: {:?}", err),    
                }
            }, 
            WindowDesc::X11(window_desc) => {
                let surface_result = unsafe {instance.create_surface(&window_desc) };
                match surface_result {
                    Ok(surface) => surface,
                    Err(err) => panic!("problem creating surface: {:?}", err),    
                }
            },
            WindowDesc::None => {
                panic!("window not provided")
            }
        };  

        let adapter = instance 
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            }).await
            .expect("Failed to find an appropriate adapter");

        let adapter_info = adapter.get_info();
        println!("Used device info: ");
        println!("name: {}", adapter_info.name);
        println!("vendor: {}", adapter_info.vendor);
        println!("device_type: {:?}", adapter_info.device_type);
        println!("device: {}", adapter_info.device);
        println!("backend: {:?}", adapter_info.backend);
        println!("driver: {:?}", adapter_info.driver);
        println!("driver_info: {}", adapter_info.driver_info);
        println!("===============");
    
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: 
                        wgpu::Features::TEXTURE_BINDING_ARRAY |
                        wgpu::Features::MAPPABLE_PRIMARY_BUFFERS |
                        wgpu::Features::PUSH_CONSTANTS , 
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: {
                        let mut limits = wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits());
                        limits.max_samplers_per_shader_stage =
                            adapter.limits().max_samplers_per_shader_stage;
                        limits.max_sampled_textures_per_shader_stage =
                            adapter.limits().max_sampled_textures_per_shader_stage;
                        limits
                    },
                },
                None,
            )
            .await
            .expect("Failed to create device");
    
        let default_configuration = default_surface_configuration(
            &surface,
            &adapter
        );

        return WindowSurface {
            surface, 
            adapter, 
            device,
            queue,
            surface_configuration: default_configuration 
        };
    }

    pub async fn new(instance: &wgpu::Instance, surface: wgpu::Surface ) -> WindowSurface {
        let adapter = instance 
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            }).await
            .expect("Failed to find an appropriate adapter");
        let adapter_info = adapter.get_info();
        //TODO may be pass it to java's logger?
        println!("Used device info: ");
        println!("name: {}", adapter_info.name);
        println!("vendor: {}", adapter_info.vendor);
        println!("device_type: {:?}", adapter_info.device_type);
        println!("device: {}", adapter_info.device);
        println!("backend: {:?}", adapter_info.backend);
        println!("driver: {:?}", adapter_info.driver);
        println!("driver_info: {}", adapter_info.driver_info);
        println!("===============");
        // Create the logical device and command queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: 
                        wgpu::Features::TEXTURE_BINDING_ARRAY |
                        wgpu::Features::MAPPABLE_PRIMARY_BUFFERS |
                        wgpu::Features::PUSH_CONSTANTS , 
                    // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                    limits: {
                        let mut limits = wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits());
                        limits.max_samplers_per_shader_stage =
                            adapter.limits().max_samplers_per_shader_stage;
                        limits.max_sampled_textures_per_shader_stage =
                            adapter.limits().max_sampled_textures_per_shader_stage;
                        limits
                    },
                },
                None,
            )
            .await
            .expect("Failed to create device");
        let default_configuration = default_surface_configuration(
            &surface,
            &adapter
        );
        return WindowSurface {
            surface, 
            adapter, 
            device,
            queue,
            surface_configuration: default_configuration 
        };
    }

    pub async fn create_window_x11(instance: &wgpu::Instance, display_ptr: *mut c_void, window_ptr: *mut c_void) -> WindowSurface {
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

        window.window.window = window_ptr as c_ulong;
        window.display.display = display_ptr as *mut c_void;
        
        let surface_result = unsafe {instance.create_surface(&window) };
        let surface = match surface_result {
            Ok(surface) => surface,
            Err(err) => panic!("problem creating surface: {:?}", err),    
        };
        WindowSurface::new(instance, surface).await
    }

    pub async fn create_window_win32(instance: &wgpu::Instance, window_ptr: *mut c_void) -> WindowSurface {
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
        window.window.hwnd = window_ptr;
        
        //let mut write_kernel = kernel.write().unwrap();
        let surface_result = unsafe {instance.create_surface(&window) };
        let surface = match surface_result {
            Ok(surface) => surface,
            Err(err) => panic!("problem creating surface: {:?}", err),    
        };

        WindowSurface::new(instance, surface).await
    }
    
    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self.surface_configuration = wgpu::SurfaceConfiguration {
            width,
             height,
            ..default_surface_configuration(&self.surface, &self.adapter)
        };
        self.surface.configure(&self.device, &self.surface_configuration);
    }

}


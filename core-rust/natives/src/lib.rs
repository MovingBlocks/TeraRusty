use context::{ROOT, Renderer};
use jni::{sys::{JNIEnv, jlong, jint}, objects::JClass};
use raw_window_handle::{XlibDisplayHandle, XlibWindowHandle, HasRawWindowHandle, RawWindowHandle,  HasRawDisplayHandle, RawDisplayHandle, Win32WindowHandle, WindowsDisplayHandle};
use std::ffi::c_void;
use futures::executor::block_on;
use std::borrow::Cow;

mod context;
mod renderer_window;
mod engine_kernel;
mod java_util;

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_TeraRusty_dispatch(_jni: JNIEnv, _class: JClass)  {
    let core = ROOT.renderer.get().unwrap();
    let pipeline = ROOT.test_pipeline.get().unwrap();

    {
        let guard = ROOT.swapchain_size.lock().unwrap(); 
        if guard.x == 0 || guard.y == 0 {
            return;
        }
    }


    let frame = core.wgpu_surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = core.wgpu_device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        rpass.set_pipeline(&pipeline);
        rpass.draw(0..3, 0..1);
    }

    core.wgpu_graphics_queue.submit(Some(encoder.finish()));
    frame.present();

}

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

async fn initialize(instance: wgpu::Instance, surface: wgpu::Surface) {
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

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });
    
    let renderer = Renderer {
        wgpu_graphics_queue: queue,
        wgpu_device : device,
        wgpu_adapter : adapter,
        wgpu_instance : instance,
        wgpu_surface : surface
    };
    let guard = ROOT.swapchain_size.lock().unwrap();
    if guard.x > 0 && guard.y > 0 {
        renderer.update_surface_size(guard.x, guard.y);
    }
    ROOT.renderer.set(renderer).unwrap();
    ROOT.test_pipeline.set(render_pipeline).unwrap();

}


#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_TeraRusty_windowSizeChanged(_jni: JNIEnv, _class: JClass, width: jint, height: jint)  {

    if width == 0 || height == 0 {
        return;
    }

    let mut guard = ROOT.swapchain_size.lock().unwrap();
    *guard = glam::UVec2::new(width as u32, height as u32);
    if let Some(renderer) = ROOT.renderer.get() {
        renderer.update_surface_size(width as u32, height as u32);
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
    let future = initialize(instance, surface);
    block_on(future);
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
pub extern "system" fn java_org_terasology_engine_rust_TeraRusty_initializeWindowWin32(_jni: JNIEnv, _class: JClass, window_ptr: jlong)  {
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
    let future = initialize(instance, surface);
    block_on(future);
}


use core::ffi::c_void;
use std::cell::RefCell;
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use futures::executor::block_on;
use jni::{
    JNIEnv,
    objects::JClass,
    sys::{jfloat, jint, jlong},
};
use wgpu::CommandEncoder;

use crate::java_util::arc_dispose_handle;
use crate::java_util::arc_to_handle;
use crate::java_util::JavaHandle;
use crate::java_util::try_arc_from_handle;
use crate::jni_support::try_throw;
use crate::resource::texture_resource::TextureResource;
use crate::ui::Rect;
use crate::ui::UserInterface;
use crate::window_surface::WindowSurface;

pub struct EngineKernel {
    pub instance: wgpu::Instance,
    pub surface: Option<WindowSurface>,
    pub user_interface: Option<UserInterface>,

    pub encoder: RefCell<Option<CommandEncoder>>,
}

impl EngineKernel {
    pub fn cmd_prepare(&mut self) -> Result<()> {
        let Some(ref mut window) = self.surface else { bail!("window is not prepared"); };

        if let Some(ui) = self.user_interface.as_mut() {
            ui.cmd_prepare();
        }
        self.encoder.replace(Some(
            window
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None }),
        ));
        Ok(())
    }

    pub fn cmd_dispatch(&mut self) -> Result<()> {
        let Some(ref mut window) = self.surface else { return Ok(()); };
        let frame = window
            .surface
            .get_current_texture()?;
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let Some(mut encoder) = self.encoder.replace(None)
            else { bail!("encoder is not prepared") };

        let Some(ui) = self.user_interface.as_mut()
            else { bail!("user interface is not prepared") };
        let frame_texture = &frame.texture;
        let size = frame_texture.size();

        ui.cmd_dispatch(
            &Rect {
                min: [0.0, 0.0],
                max: [size.width as f32, size.height as f32],
            },
            &view,
            &window.device,
            &window.queue,
            &mut encoder,
        );

        window.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
        Ok(())
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_drop(
        _jni: JNIEnv,
        _class: JClass,
        ptr: jlong,
    ) {
        EngineKernel::drop_handle(ptr);
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_create(
        _jni: JNIEnv,
        _class: JClass,
    ) -> jlong {
        EngineKernel::to_handle(Arc::new(RefCell::new(EngineKernel {
            instance: wgpu::Instance::default(),
            surface: None,
            user_interface: None,
            encoder: RefCell::new(None),
        })))
    }
    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_resizeSurface(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
        width: jint,
        height: jint,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let mut kernel = kernel_arc.try_borrow_mut()?;
            fn resolve_helper(
                kernel: &mut EngineKernel,
            ) -> (&mut Option<WindowSurface>, &mut Option<UserInterface>) {
                return (&mut kernel.surface, &mut kernel.user_interface);
            }
            let (surface, ui) = resolve_helper(&mut kernel);

            let Some(surface) = surface.as_mut() else { bail!("surface not initialized"); };
            surface.resize_surface(width, height);

            Ok(if let Some(ui) = ui.as_mut() {
                ui.resize_surface(&surface.device, &glam::IVec2::new(width, height))
            })
        })
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdDispatch(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let mut kernel = kernel_arc.borrow_mut();
            kernel.cmd_dispatch()
        })
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdPrepare(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let mut kernel = kernel_arc.borrow_mut();
            kernel.cmd_prepare()
        })
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceX11(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
        display_ptr: jlong,
        window_ptr: jlong,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let mut kernel = kernel_arc.try_borrow_mut()?;
            kernel.surface = Some(block_on(WindowSurface::create_window_x11(
                &kernel.instance,
                display_ptr as *mut c_void,
                window_ptr as *mut c_void,
            ))?);
            kernel.initialize_subsystems()
        })
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_initSurfaceWin32(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
        _display_ptr: jlong,
        window_ptr: jlong,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let mut kernel = kernel_arc.borrow_mut();
            kernel.surface = Some(block_on(WindowSurface::create_window_win32(
                &kernel.instance,
                window_ptr as *mut c_void,
            ))?);
            kernel.initialize_subsystems()
        })
    }

    fn initialize_subsystems(&mut self) -> Result<()> {
        let Some(ref window) = &self.surface.as_ref()
            else { bail!("Surface is not prepared") };
        self.user_interface = Some(UserInterface::new(
            &window.device,
            &window.surface_configuration,
        ));
        Ok(())
    }

    // User Interface
    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdUISetCrop(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
        min_x: jfloat,
        min_y: jfloat,
        max_x: jfloat,
        max_y: jfloat,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let mut kernel = kernel_arc.borrow_mut();
            let Some(ui) = kernel.user_interface.as_mut() else { bail!("surface invalid"); };

            ui.cmd_set_crop(Some(Rect {
                min: [min_x, min_y],
                max: [max_x, max_y],
            }));
            Ok(())
        })
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdUIClearCrop(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let mut kernel = kernel_arc.borrow_mut();
            let Some(ui) = kernel.user_interface.as_mut() else { bail!("surface invalid"); };
            ui.cmd_set_crop(None);
            Ok(())
        })
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub extern "system" fn Java_org_terasology_engine_rust_EngineKernel_00024JNI_cmdUIDrawTexture(
        mut jni: JNIEnv,
        _class: JClass,
        kernel_ptr: jlong,
        tex_ptr: jlong,
        uv_min_x: jfloat,
        uv_min_y: jfloat,
        uv_max_x: jfloat,
        uv_max_y: jfloat,
        pos_min_x: jfloat,
        pos_min_y: jfloat,
        pos_max_x: jfloat,
        pos_max_y: jfloat,
        tint_color: jint,
    ) {
        try_throw(&mut jni, |_| {
            let kernel_arc = EngineKernel::from_handle(kernel_ptr)?;
            let text_resource_arc = TextureResource::from_handle(tex_ptr)?;
            let mut kernel = kernel_arc.borrow_mut();
            fn resolve_ui_window(
                kernel: &mut EngineKernel,
            ) -> Result<(
                &mut RefCell<Option<CommandEncoder>>,
                &mut WindowSurface,
                &mut UserInterface,
            )> {
                return Ok((
                    &mut kernel.encoder,
                    kernel.surface.as_mut().ok_or(anyhow!("surface is not prepared"))?,
                    kernel.user_interface.as_mut().ok_or(anyhow!("user interface is not prepared"))?
                ));
            }
            let (_encoder, window, ui) = resolve_ui_window(&mut kernel)?;

            ui.cmd_draw_texture(
                &window.queue,
                &window.device,
                &text_resource_arc,
                &Rect {
                    min: [uv_min_x, uv_min_y],
                    max: [uv_max_x, uv_max_y],
                },
                &Rect {
                    min: [pos_min_x, pos_min_y],
                    max: [pos_max_x, pos_max_y],
                },
                tint_color as u32,
            );
            Ok(())
        })
    }
}

impl JavaHandle<Arc<RefCell<EngineKernel>>> for EngineKernel {
    fn from_handle(ptr: jlong) -> Result<Arc<RefCell<EngineKernel>>> {
        try_arc_from_handle(ptr).map_err(|_| anyhow!("Unable to get Engine Kernel handle by ptr"))
    }

    fn to_handle(from: Arc<RefCell<EngineKernel>>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<RefCell<EngineKernel>>(ptr);
    }
}
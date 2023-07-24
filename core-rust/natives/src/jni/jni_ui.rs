use jni::{sys::{jlong, jfloat, jint}, objects::{JClass, JByteBuffer, JObject}, JNIEnv};
use crate::{engine_kernel::EngineKernel, ui::JavaHandle, math::rect::Rect, resource::texture_resource::TextureResource};

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_UIRenderer_00024JNI_cmdUISetCrop(_jni: JNIEnv, _class: JClass,
        kernel_ptr: jlong, min_x: jfloat, min_y: jfloat, max_x: jfloat, max_y: jfloat ) {
    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
    let mut ui = kernel.user_interface.borrow_mut();
    ui.cmd_set_crop(Some(Rect {
        min: [min_x, min_y],
        max: [max_x, max_y]
    }));
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_UIRenderer_00024JNI_cmdUIClearCrop<'local>(env: JNIEnv<'local>, _class: JClass, kernel_ptr: jlong) {
    let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
    let mut ui = kernel.user_interface.borrow_mut();
    ui.cmd_set_crop(None);
}

#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_UIRenderer_00024JNI_cmdUIDrawTexture<'local>(mut env: JNIEnv<'local>, _class: JClass, 
        kernel_ptr: jlong,
        tex_ptr: jlong,
        uv_min_x: jfloat, uv_min_y: jfloat, uv_max_x: jfloat, uv_max_y: jfloat,
        pos_min_x: jfloat, pos_min_y: jfloat, pos_max_x: jfloat, pos_max_y: jfloat,
        tint_color: jint) {
        let Some(kernel) = EngineKernel::from_handle(kernel_ptr) else { panic!("kernel invalid") };
        let Some(texture_resource) = TextureResource::from_handle(tex_ptr) else {panic!("invalid tex resource")};
       
        let surface = kernel.window_surface.lock().expect("failed to resolve surface");
        let mut ui = kernel.user_interface.borrow_mut();
        ui.cmd_draw_texture(
            &surface.queue,
            &surface.device,
            &texture_resource,
            &Rect {
                min: [uv_min_x, uv_min_y],
                max: [uv_max_x, uv_max_y]
            },
            &Rect {
                min: [pos_min_x, pos_min_y],
                max: [pos_max_x, pos_max_y]
            },
            tint_color as u32
        );


}


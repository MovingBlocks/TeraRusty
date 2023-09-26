use jni::{JNIEnv, objects::JClass, sys::jlong};

use crate::{resource::chunk_mesh_resource::ChunkMeshResource, ui::JavaHandle};



#[no_mangle]
pub extern "system" fn Java_org_terasology_engine_rust_resource_ChunkGeometry_00024JNI_drop<'local>(mut env: JNIEnv<'local>, _class: JClass, geom_ptr: jlong) {
    ChunkMeshResource::drop_handle(geom_ptr);
}

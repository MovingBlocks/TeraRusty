use jni::sys::jlong;
use std::sync::Arc;

pub trait JavaHandle<T> {
    fn from_handle(ptr: jlong) -> Option<T>;
    fn to_handle(from: T) -> jlong;
    fn drop_handle(ptr: jlong); 
}

pub trait JavaHandleContainer<T> {
    fn from_handle(&self, ptr: jlong) -> Option<T>;
    fn to_handle(&self, from: T) -> jlong;

}

pub fn arc_from_handle<T>(ptr: jlong) -> Option<Arc<T>> {
    if ptr == 0 {
        panic!("invalid handle");
    }
    
    unsafe { 
        let kernel = ptr as *const T;
        Arc::increment_strong_count(kernel);
        Some(Arc::from_raw(kernel))
    }
}

pub fn arc_to_handle<T>(from: Arc<T>) -> jlong {
    Arc::into_raw(from) as jlong 
}

pub fn arc_dispose_handle<T>(ptr: jlong) {
    if ptr == 0 {
        panic!("double free");
    }
    let kernel = ptr as *const T;
    drop(unsafe { Arc::from_raw(kernel) });
}



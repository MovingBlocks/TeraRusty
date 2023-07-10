use jni::sys::jlong;
use std::sync::Arc;

pub trait JInstance {
    type Item;
    type JavaType;

    fn from_jlong(ptr: jlong) -> Self::Item;
    fn to_jlong(from: Self::Item) -> jlong;

    fn java_create(from: Self) ->  jlong;
    fn java_dispose(ptr: jlong);
}

pub trait JArcInstance {
    type Item;
    type JavaType;

    fn from_jlong(ptr: jlong) -> Arc<Self::Item> {
        unsafe { 
            let kernel = ptr as *const Self::Item;
            Arc::increment_strong_count(kernel);
            Arc::from_raw(kernel) 
        }
    }

    fn to_jlong(from: Arc<Self::Item>) -> jlong {
       Arc::into_raw(from) as jlong 
    }
    
    fn java_create(from: Self::Item) ->  jlong {
        let renderer = Arc::new(from);
        Arc::into_raw(renderer) as jlong
    }

    fn java_dispose(ptr: jlong) {
        if ptr == 0 {
            return;
        }
        let kernel = ptr as *const Self::Item;
        drop(unsafe { Arc::from_raw(kernel) });
    }
}


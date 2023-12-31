use jni::{sys::{jlong}, objects::{JObject, JValue}};
use std::sync::Arc;
use std::marker::PhantomData;

use jni::JNIEnv;

pub trait JavaHandle<T> {
    fn from_handle(ptr: jlong) -> Option<T>;
    fn to_handle(from: T) -> jlong;
    fn drop_handle(ptr: jlong); 
}

pub trait JavaHandleContainer<T> {
    fn from_handle(&self, ptr: jlong) -> Option<T>;
    fn to_handle(&self, from: T) -> jlong;

}

//pub struct JomlVector2<'a, T> {
//    obj: &'a JObject<'a>,
//    phantom: PhantomData<T> 
//}
//
//pub fn joml_vec2<'a, T>(o: &'a JObject<'a>) -> JomlVector2<'a, T> {
//    JomlVector2::<'a, T> {
//       obj: o,
//       phantom: PhantomData
//    }
//} 
//
//
//impl <'a> JomlVector2<'a, f32> {
//    pub fn set(&mut self,env: &mut JNIEnv, x: f32, y: f32) {
//        env.set_field(&self.obj, "x", "F", JValue::Float(x)).expect("failed to set x");
//        env.set_field(&self.obj, "y", "F", JValue::Float(y)).expect("failed to set y");
//    }
//    pub fn get(&mut self, env: &mut JNIEnv) -> [f32;2] {
//        let x = env.get_field(&self.obj, "x", "F").expect("failed to set x").f().expect("expect float type");
//        let y = env.get_field(&self.obj, "y", "F").expect("failed to set y").f().expect("expect float type");
//        return [x, y]
//    }
//} 
//
//impl <'a> JomlVector2<'a, i32> {
//    pub fn set(&mut self,mut env: JNIEnv, x: i32, y: i32) {
//        env.set_field(&self.obj, "x", "I", JValue::Int(x)).expect("failed to set x");
//        env.set_field(&self.obj, "y", "I", JValue::Int(y)).expect("failed to set y");
//    }
//    pub fn get(&mut self, mut env: JNIEnv) -> [i32;2] {
//        let x = env.get_field(&self.obj, "x", "I").expect("failed to set x").i().expect("expect float type");
//        let y = env.get_field(&self.obj, "y", "I").expect("failed to set y").i().expect("expect float type");
//        return [x,y]
//    }
//} 

pub type JNIResult<T> = Result<T, JNIError >;

pub enum JNIError { 
    Generic(String),
    //NullException(String)
}

pub fn to_java_exception<'local>(mut env: JNIEnv<'local>, error: &JNIError) -> jni::errors::Result<JObject<'local>> {
    match error {
        JNIError::Generic(err) => {
            let error_string = env.new_string(err)?;
            env.new_object(
                "java/lang/Error",
                "(Ljava/lang/String;)V",
                &[
                    (&error_string).into(),
                ],
            )
        }
    }
}

pub fn set_joml_vector2f(mut env: JNIEnv, o: &mut JObject, x: f32, y: f32) {
    env.set_field(&o, "x", "F", JValue::Float(x)).expect("failed to set x");
    env.set_field(&o, "y", "F", JValue::Float(y)).expect("failed to set y");
}

pub fn set_joml_vector3f(mut env: JNIEnv, o: &mut JObject, x: f32, y: f32, z: f32) {
    env.set_field(&o, "x", "F", JValue::Float(x)).expect("failed to set x");
    env.set_field(&o, "y", "F", JValue::Float(y)).expect("failed to set y");
    env.set_field(&o, "z", "F", JValue::Float(z)).expect("failed to set z");
}

pub fn set_joml_vector4f(mut env: JNIEnv, o: &mut JObject, x: f32, y: f32, z: f32, w: f32) {
    env.set_field(&o, "x", "F", JValue::Float(x)).expect("failed to set x");
    env.set_field(&o, "y", "F", JValue::Float(y)).expect("failed to set y");
    env.set_field(&o, "z", "F", JValue::Float(z)).expect("failed to set z");
    env.set_field(&o, "w", "F", JValue::Float(w)).expect("failed to set w");
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



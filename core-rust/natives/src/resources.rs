use std::sync::{Weak, Arc};
use std::rc::Rc;
use jni::sys::jlong;
use slotmap::{DefaultKey, SlotMap};
use smallvec::SmallVec;
use std::sync::RwLock;
use std::cell::RefCell;
use crate::engine_kernel::EngineKernel;
use crate::java_util::{JavaHandle, arc_to_handle, arc_from_handle, arc_dispose_handle};
    
pub struct ResourceManager {    
    geometry: SlotMap<DefaultKey, GeometryHandle>
}

impl ResourceManager {
    pub fn new() -> Self {
        return ResourceManager {
            geometry: SlotMap::new()
        }
    }
}


trait AllocResource<T> {
    fn create(&self) -> T;
}


enum Semantic {
    POSITION,
    NORMAL,
    COLOR,
    TEXCOORD0,
    TEXCOORD1,
    TEXCOORD2,
    TEXCOORD3,
    TEXCOORD4,
    TEXCOORD5,
}

impl Semantic {
    pub fn light0() -> Self { Semantic::TEXCOORD3 }
}

pub struct ResourceStream {
    buf: Rc<wgpu::Buffer>,
    semantic: Semantic
}

pub struct IndexStream {
    buf: Rc<wgpu::Buffer>
}

pub struct GeometryHandle {
    kernel: Weak<RwLock<EngineKernel>>,
    stream: SmallVec<[ResourceStream; 15]>,
    index_stream: Option<IndexStream>
}

impl JavaHandle<Arc<RefCell<GeometryHandle>>> for GeometryHandle {

    fn from_handle(ptr: jlong) -> Option<Arc<RefCell<GeometryHandle>>> {
        arc_from_handle(ptr)  
    }

    fn to_handle(from: Arc<RefCell<GeometryHandle>>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<RefCell<GeometryHandle>>(ptr); 
    }
}

impl GeometryHandle {
    fn streams(&self) -> &[ResourceStream] {
        self.stream.as_slice() 
    }

    fn new(kernel: &Arc<RwLock<EngineKernel>>) -> Arc<RefCell<Self>> {
        Arc::new(RefCell::new(Self {
            kernel: Arc::downgrade(kernel),
            stream: SmallVec::new(), 
            index_stream: None 
        }))
    }
}


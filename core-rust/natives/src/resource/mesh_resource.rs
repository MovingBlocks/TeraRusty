use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Weak};

use anyhow::Result;
use jni::sys::jlong;
use smallvec::SmallVec;

use crate::engine_kernel::EngineKernel;
use crate::java_util::{arc_dispose_handle, arc_to_handle, JavaHandle, try_arc_from_handle};

//pub struct ResourceManager {
//    geometry: SlotMap<DefaultKey, GeometryHandle>
//}
//
//impl ResourceManager {
//    pub fn new() -> Self {
//        return ResourceManager {
//            geometry: SlotMap::new()
//        }
//    }
//}

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
    pub fn light0() -> Self {
        Semantic::TEXCOORD3
    }
}

pub struct ResourceStream {
    buf: Rc<wgpu::Buffer>,
    semantic: Semantic,
}

pub struct IndexStream {
    buf: Rc<wgpu::Buffer>,
}

pub struct GeometryResource {
    kernel: Weak<RefCell<EngineKernel>>,
    stream: SmallVec<[ResourceStream; 15]>,
    index_stream: Option<IndexStream>,
}

impl JavaHandle<Arc<RefCell<GeometryResource>>> for GeometryResource {
    fn from_handle(ptr: jlong) -> Result<Arc<RefCell<GeometryResource>>> {
        try_arc_from_handle(ptr)
    }

    fn to_handle(from: Arc<RefCell<GeometryResource>>) -> jlong {
        arc_to_handle(from)
    }

    fn drop_handle(ptr: jlong) {
        arc_dispose_handle::<GeometryResource>(ptr);
    }
}

impl GeometryResource {
    fn streams(&self) -> &[ResourceStream] {
        self.stream.as_slice()
    }

    fn new(kernel: &Arc<RefCell<EngineKernel>>) -> Arc<RefCell<Self>> {
        Arc::new(RefCell::new(Self {
            kernel: Arc::downgrade(kernel),
            stream: SmallVec::new(),
            index_stream: None,
        }))
    }
}

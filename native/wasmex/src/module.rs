use rustler::{resource::ResourceArc, types::binary::Binary, NifResult};
use std::sync::Mutex;

use wasmer::{wat2wasm, Module, Store};

use crate::atoms;

pub struct ModuleResource {
    pub module: Mutex<Module>,
}

#[derive(NifTuple)]
pub struct ModuleResourceResponse {
    ok: rustler::Atom,
    resource: ResourceArc<ModuleResource>,
}

#[rustler::nif(name = "module_compile")]
pub fn compile(binary: Binary) -> NifResult<ModuleResourceResponse> {
    let bytes = binary.as_slice();
    let bytes = wat2wasm(bytes).map_err(|e| {
        rustler::Error::Term(Box::new(format!(
            "Error while parsing bytes: {}.",
            e.to_string()
        )))
    })?;
    let store = Store::default();
    match Module::new(&store, bytes) {
        Ok(module) => {
            let resource = ResourceArc::new(ModuleResource {
                module: Mutex::new(module),
            });
            Ok(ModuleResourceResponse {
                ok: atoms::ok(),
                resource,
            })
        }
        Err(e) => Err(rustler::Error::Term(Box::new(format!(
            "Could not compile module: {:?}",
            e
        )))),
    }
}

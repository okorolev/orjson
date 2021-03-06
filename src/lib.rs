// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#![feature(custom_attribute)]
#![feature(core_intrinsics)]

#[macro_use]
extern crate pyo3;

extern crate serde;
extern crate serde_json;
extern crate smallvec;

use pyo3::prelude::*;
use pyo3::ToPyPointer;
use std::ptr::NonNull;

mod decode;
mod encode;
mod exc;
mod typeref;

#[pymodule]
fn orjson(py: Python, m: &PyModule) -> PyResult<()> {
    typeref::init_typerefs();
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_function!(dumps))?;
    m.add_wrapped(wrap_function!(loads))?;
    m.add("JSONDecodeError", py.get_type::<exc::JSONDecodeError>())?;
    m.add("JSONEncodeError", py.get_type::<exc::JSONEncodeError>())?;
    Ok(())
}

/// loads(obj, /)
/// --
///
/// Deserialize JSON to Python objects.
#[pyfunction]
pub fn loads(py: Python, obj: PyObject) -> PyResult<PyObject> {
    decode::deserialize(py, obj.as_ptr())
}

/// dumps(obj, default, /)
/// --
///
/// Serialize Python objects to JSON.
#[pyfunction]
pub fn dumps(py: Python, obj: PyObject, default: Option<PyObject>) -> PyResult<PyObject> {
    let pydef: Option<NonNull<pyo3::ffi::PyObject>>;
    if default.is_some() {
        pydef = Some(unsafe { NonNull::new_unchecked(default.unwrap().as_ptr()) });
    } else {
        pydef = None
    };
    encode::serialize(py, obj.as_ptr(), pydef)
}

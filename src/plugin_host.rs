use std::ffi::CString;
use pyo3::prelude::*;
use std::fs;

pub struct MiragePluginHost {}

impl MiragePluginHost {
    pub fn new() -> MiragePluginHost {
        MiragePluginHost {}
    }

    pub fn run_plugins(&self) {
        Python::with_gil(|py| {
            let paths = fs::read_dir("plugins/").unwrap();

            for path in paths {
                let path = path.unwrap().path();
                let p = fs::read_to_string(&path).unwrap();
                let fun: Py<PyAny> = PyModule::from_code_bound(py, p.as_str(), "", "")
                    .unwrap()
                    .getattr("run")
                    .unwrap()
                    .into();
                fun.call0(py).unwrap();
            }
        })
    }
}
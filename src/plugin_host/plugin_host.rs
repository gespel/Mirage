use std::ffi::CString;
use pyo3::prelude::*;
use std::fs;
use std::fs::ReadDir;

pub struct MiragePluginHost {
    num_active_plugins: usize,
    plugin_dir_path: ReadDir,
    programs: Vec<String>,
}

impl MiragePluginHost {
    pub fn new() -> MiragePluginHost {
        let plugin_dir_path =  fs::read_dir("plugins/").expect("unable to read plugins dir");
        let mut num_active_plugins = 0;
        let mut programs: Vec<String> = Vec::new();
        for path in &plugin_dir_path {
            num_active_plugins += 1;
            let p = fs::read_to_string(&path).expect(
                format!(
                    "Unable to read plugin {}",
                    path.display()
                ).as_str()
            );
            programs.push(p);
        }

        MiragePluginHost {
            num_active_plugins,
            plugin_dir_path,
            programs,
        }
    }

    pub fn run_plugins(&mut self) {
        Python::with_gil(|py| {
            for p in self.plugin_dir {


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
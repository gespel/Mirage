use std::ffi::CString;
use pyo3::prelude::*;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;

pub struct MiragePluginHost {
    pub num_active_plugins: usize,
    pub plugin_dir_path: PathBuf,
    pub programs: Vec<String>,
}

impl MiragePluginHost {
    pub fn new(plugin_folder: &str) -> MiragePluginHost {
        let plugin_dir_path =  fs::read_dir(plugin_folder).expect("unable to read plugins dir");

        let mut num_active_plugins = 0;
        let mut programs: Vec<String> = Vec::new();
        for entry in plugin_dir_path {
            let entry = entry.expect("unable to read plugins dir entry");
            num_active_plugins += 1;
            let p = fs::read_to_string(entry.path()).expect(
                format!(
                    "Unable to read plugin {}",
                    entry.path().display()
                ).as_str()
            );
            programs.push(p);
        }

        MiragePluginHost {
            num_active_plugins,
            plugin_dir_path: plugin_folder.parse().expect("unable to parse plugins dir"),
            programs,
        }
    }

    pub fn run_plugins(&mut self) {
        Python::with_gil(|py| {
            for p in &self.programs {
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
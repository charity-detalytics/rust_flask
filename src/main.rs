use pyo3::prelude::*;
use std::process::Command;
use std::str;
use std::env;

fn main() -> PyResult<()> {
    let output = Command::new("pipenv")
        .arg("--venv")
        .output()
        .expect("failed to execute process");

    let venv_path = str::from_utf8(&output.stdout).unwrap().trim();

    // Construct the PYTHONPATH. Adjust the subpath as needed for your setup.
    let pythonpath = format!("{}/lib/python3.11/site-packages", venv_path); // Adjust python version

    // Set the PYTHONPATH environment variable
    env::set_var("PYTHONPATH", &pythonpath);
    println!("Set PYTHONPATH to {}", pythonpath);

    let py_app = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/flask_hello/app.py"));
    let flask_run = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: Py<PyAny> = PyModule::from_code(py, py_app, "", "")?
            .getattr("run")?
            .into();
        app.call0(py)
    });

    flask_run?;
    Ok(())
}

use pyo3::prelude::*;

fn main() -> PyResult<()> {
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

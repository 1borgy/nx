// use pyo3::{
//     prelude::*,
//     types::{IntoPyDict, PyDict, PyList},
// };
//
// mod load;
//
// #[pyfunction]
// fn load_qb(py: Python<'_>, filepath: &str) -> PyResult<Py<PyDict>> {
//     let parts = vec![("ncomp", vec!["hi"]), ("nodearray", vec!["hi"])];
//     let parts = parts.into_py_dict(py);
//
//     Ok(parts.into())
// }
//
// #[pymodule]
// fn nx_blender(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(load_qb, m)?)?;
//     Ok(())
// }

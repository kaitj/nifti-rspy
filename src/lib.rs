use nifti::{object::GenericNiftiObject, IntoNdArray, NiftiObject, ReaderOptions};
use numpy::{IntoPyArray, PyArray};
use pyo3::{exceptions::PyIOError, prelude::*};

#[pyfunction]
fn load_nifti(py: Python, path: &str) -> PyResult<Py<PyArray<f32, numpy::IxDyn>>> {
    // Read file
    let obj: GenericNiftiObject<nifti::InMemNiftiVolume> =
        ReaderOptions::new().read_file(path).unwrap();

    // Convert volume to ndarray
    let volume = obj.into_volume();
    let arr = volume.into_ndarray::<f32>().unwrap();

    // Convert ndarray to PyArray
    let py_array: Bound<PyArray<f32, ndarray::Dim<ndarray::IxDynImpl>>> =
        arr.into_pyarray_bound(py).to_owned();
    Ok(py_array.into())
}

/// A Python module implemented in Rust.
#[pymodule]
fn nifti_rspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_nifti, m)?)?;
    Ok(())
}

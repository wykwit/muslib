use pyo3::prelude::*;

use crate::algs::*;

#[pymodule]
/// TODO: This is a Python docstring of this module.
fn muslib(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<io::MonoLoader>()?;
    m.add_class::<io::MonoWriter>()?;
    m.add_class::<synth::Synthesizer>()?;
    Ok(())
}
